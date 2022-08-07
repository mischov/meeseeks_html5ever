use std::borrow::Cow;
use std::collections::HashSet;
use std::default::Default;

use html5ever::tree_builder::{ElementFlags, NodeOrText, QuirksMode, TreeSink};
use html5ever::{Attribute, QualName};
use markup5ever::ExpandedName;

use tendril::StrTendril;

use rustler::{Encoder, Env, Term};

use self::NodeEnum::{Comment, Data, Doctype, Document, Element, ProcessingInstruction, Text};

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Id(usize);

#[derive(Debug)]
enum Parent {
    Some(Id),
    None,
}

impl Parent {
    fn is_some(&self) -> bool {
        match *self {
            Parent::Some(_) => true,
            Parent::None => false,
        }
    }
}

#[derive(Debug)]
enum ScriptOrStyle {
    Script,
    Style,
    Neither,
}

#[derive(Debug)]
enum DataType {
    Script,
    Style,
    Cdata,
}

#[derive(Debug)]
enum NodeEnum {
    Comment(StrTendril),
    Data(DataType, StrTendril),
    Doctype(StrTendril, StrTendril, StrTendril),
    Document,
    Element(QualName, Vec<Attribute>, bool),
    ProcessingInstruction(StrTendril, StrTendril),
    Text(StrTendril),
}

impl NodeEnum {
    fn script_or_style(&self) -> ScriptOrStyle {
        match *self {
            Element(ref name, ..) => match name.expanded() {
                expanded_name!(html "script") => ScriptOrStyle::Script,
                expanded_name!(html "style") => ScriptOrStyle::Style,
                _ => ScriptOrStyle::Neither,
            },
            _ => ScriptOrStyle::Neither,
        }
    }

    fn append_text(&mut self, text: &str) -> bool {
        match *self {
            Text(ref mut current) | Data(_, ref mut current) => {
                current.push_slice(text);
                true
            }
            _ => false,
        }
    }
}

#[derive(Debug)]
struct Node {
    parent: Parent,
    id: Id,
    children: Vec<Id>,
    last_string: bool,
    node: NodeEnum,
}

impl Node {
    fn new(id: Id, node: NodeEnum) -> Node {
        Node {
            parent: Parent::None,
            id: id,
            children: vec![],
            last_string: false,
            node: node,
        }
    }

    fn index_of_child(&self, child: Id) -> Option<usize> {
        self.children.iter().position(|&x| x == child)
    }
}

#[derive(Debug)]
pub struct FlatDom {
    nodes: Vec<Node>,
}

impl FlatDom {
    fn node(&self, id: Id) -> &Node {
        &self.nodes[id.0]
    }

    fn node_mut(&mut self, id: Id) -> &mut Node {
        &mut self.nodes[id.0]
    }

    fn add_node(&mut self, node: NodeEnum) -> Id {
        let id = Id(self.nodes.len());
        self.nodes.push(Node::new(id, node));
        id
    }

    fn append_node(&mut self, parent: Id, child: Id) {
        self.node_mut(child).parent = Parent::Some(parent);
        let parent_node = self.node_mut(parent);
        parent_node.children.push(child);
        parent_node.last_string = false;
    }

    fn append_text(&mut self, parent: Id, text: StrTendril) {
        if self.node(parent).last_string {
            match self.node(parent).children.last() {
                Some(&child) => self.node_mut(child).node.append_text(&text),
                _ => unreachable!(),
            };
        } else {
            let child = match self.node(parent).node.script_or_style() {
                ScriptOrStyle::Script => self.add_node(Data(DataType::Script, text)),
                ScriptOrStyle::Style => self.add_node(Data(DataType::Style, text)),
                ScriptOrStyle::Neither => self.add_node(Text(text)),
            };
            self.node_mut(child).parent = Parent::Some(parent);
            let parent_node = self.node_mut(parent);
            parent_node.children.push(child);
            parent_node.last_string = true;
        }
    }

    fn get_parent_and_index(&self, child: Id) -> Option<(Id, usize)> {
        let maybe_parent = &self.node(child).parent;
        match *maybe_parent {
            Parent::None => None,
            Parent::Some(parent) => match self.node(parent).index_of_child(child) {
                Some(i) => Some((parent, i)),
                None => panic!("have parent but not in parent"),
            },
        }
    }

    fn remove_from_parent(&mut self, child: Id) {
        if let Some((parent, i)) = self.get_parent_and_index(child) {
            self.node_mut(parent).children.remove(i);
            let child = self.node_mut(child);
            child.parent = Parent::None;
        }
    }
}

impl Default for FlatDom {
    fn default() -> FlatDom {
        FlatDom {
            nodes: vec![Node::new(Id(0), Document)],
        }
    }
}

impl TreeSink for FlatDom {
    type Output = FlatDom;
    type Handle = Id;

    fn finish(self) -> Self::Output {
        self
    }

    // Not supported
    fn parse_error(&mut self, _msg: Cow<'static, str>) {}

    fn get_document(&mut self) -> Self::Handle {
        Id(0)
    }

    fn get_template_contents(&mut self, target: &Self::Handle) -> Self::Handle {
        if let Element(_, _, true) = self.node(*target).node {
            // Use template element as document fragment
            target.clone()
        } else {
            panic!("not a template element!")
        }
    }

    // Not supported
    fn set_quirks_mode(&mut self, _mode: QuirksMode) {}

    fn same_node(&self, x: &Self::Handle, y: &Self::Handle) -> bool {
        x == y
    }

    fn elem_name(&self, target: &Self::Handle) -> ExpandedName {
        if let Element(ref name, ..) = self.node(*target).node {
            name.expanded()
        } else {
            panic!("not an element!")
        }
    }

    fn create_element(
        &mut self,
        name: QualName,
        attrs: Vec<Attribute>,
        flags: ElementFlags,
    ) -> Self::Handle {
        self.add_node(Element(name, attrs, flags.template))
    }

    fn create_comment(&mut self, text: StrTendril) -> Self::Handle {
        if text.starts_with("[CDATA[") && text.ends_with("]]") {
            let data = StrTendril::from_slice(&text[7..(text.len() - 2)]);
            self.add_node(Data(DataType::Cdata, data))
        } else {
            self.add_node(Comment(text))
        }
    }

    fn create_pi(&mut self, target: StrTendril, data: StrTendril) -> Self::Handle {
        self.add_node(ProcessingInstruction(target, data))
    }

    fn append(&mut self, parent: &Self::Handle, child: NodeOrText<Self::Handle>) {
        match child {
            NodeOrText::AppendNode(node) => self.append_node(*parent, node),
            NodeOrText::AppendText(text) => self.append_text(*parent, text),
        };
    }

    fn append_before_sibling(&mut self, sibling: &Self::Handle, child: NodeOrText<Self::Handle>) {
        let (parent, i) = self
            .get_parent_and_index(*sibling)
            .expect("append_before_sibling called on node without parent");

        let child = match (child, i) {
            // No previous node
            (NodeOrText::AppendText(text), 0) => self.add_node(Text(text)),

            // Check for text node before insertion point, append if there is
            (NodeOrText::AppendText(text), i) => {
                let prev = self.node(parent).children[i - 1];
                if self.node_mut(prev).node.append_text(&text) {
                    return;
                }
                self.add_node(Text(text))
            }

            // Tree builder promises no text no *after* insertion point

            // Any other kind of node
            (NodeOrText::AppendNode(node), _) => node,
        };

        if self.node(child).parent.is_some() {
            self.remove_from_parent(child);
        }

        self.node_mut(child).parent = Parent::Some(parent);
        self.node_mut(parent).children.insert(i, child);
    }

    fn append_based_on_parent_node(
        &mut self,
        element: &Self::Handle,
        prev_element: &Self::Handle,
        child: NodeOrText<Self::Handle>,
    ) {
        let has_parent = self.node(*element).parent.is_some();
        if has_parent {
            self.append_before_sibling(element, child);
        } else {
            self.append(prev_element, child);
        }
    }

    fn append_doctype_to_document(
        &mut self,
        name: StrTendril,
        public_id: StrTendril,
        system_id: StrTendril,
    ) {
        let doctype = self.add_node(Doctype(name, public_id, system_id));
        self.append_node(Id(0), doctype);
    }

    fn add_attrs_if_missing(&mut self, target: &Self::Handle, attrs: Vec<Attribute>) {
        let target_node = self.node_mut(*target);
        let target_attrs = if let Element(_, ref mut attrs, ..) = target_node.node {
            attrs
        } else {
            panic!("not an element")
        };

        let existing_names = target_attrs
            .iter()
            .map(|e| e.name.clone())
            .collect::<HashSet<_>>();
        target_attrs.extend(
            attrs
                .into_iter()
                .filter(|attr| !existing_names.contains(&attr.name)),
        );
    }

    fn remove_from_parent(&mut self, target: &Self::Handle) {
        self.remove_from_parent(*target);
    }

    fn reparent_children(&mut self, node: &Self::Handle, new_parent: &Self::Handle) {
        let children = self.node(*node).children.clone();
        for child in &children {
            self.remove_from_parent(*child);
            self.append_node(*new_parent, *child);
        }
    }
}

// NIF Encoding

mod atoms {
    atoms! {
        nil,

        parent,
        id,
        content,
        name,
        public,
        system,
        namespace,
        tag,
        attributes,
        children,
        target,
        data,

        type_ = "type",
        script,
        style,
        cdata,

        id_counter,
        roots,
        nodes,

        __struct__,
        document = "Elixir.Meeseeks.Document",
        document_comment = "Elixir.Meeseeks.Document.Comment",
        document_data = "Elixir.Meeseeks.Document.Data",
        document_doctype = "Elixir.Meeseeks.Document.Doctype",
        document_element = "Elixir.Meeseeks.Document.Element",
        document_pi = "Elixir.Meeseeks.Document.ProcessingInstruction",
        document_text = "Elixir.Meeseeks.Document.Text",
    }
}

// QualName and StrTendril

// Zero-cost wrapper types which makes it possible to implement
// Encoder for these externally defined types.
// Unsure if this is a great way of doing it, but it's the way
// that produced the cleanest and least noisy code.

struct QNW<'a>(&'a QualName);

impl<'b> Encoder for QNW<'b> {
    fn encode<'a>(&self, env: Env<'a>) -> Term<'a> {
        let local: &str = &*self.0.local;
        local.encode(env)
    }
}

struct STW<'a>(&'a StrTendril);

impl<'b> Encoder for STW<'b> {
    fn encode<'a>(&self, env: Env<'a>) -> Term<'a> {
        let data: &str = &*self.0;
        data.encode(env)
    }
}

// Id

impl Encoder for Id {
    fn encode<'a>(&self, env: Env<'a>) -> Term<'a> {
        self.0.encode(env)
    }
}

// Parent

impl Encoder for Parent {
    fn encode<'a>(&self, env: Env<'a>) -> Term<'a> {
        match *self {
            Parent::None => atoms::nil().encode(env),
            Parent::Some(id) => {
                if id == Id(0) {
                    atoms::nil().encode(env)
                } else {
                    id.encode(env)
                }
            }
        }
    }
}

// DataType

impl Encoder for DataType {
    fn encode<'a>(&self, env: Env<'a>) -> Term<'a> {
        match *self {
            DataType::Script => atoms::script().encode(env),
            DataType::Style => atoms::style().encode(env),
            DataType::Cdata => atoms::cdata().encode(env),
        }
    }
}

// Node

fn split_ns_and_tag(ns_tag: &str) -> (&str, &str) {
    let first_colon = ns_tag.find(':').unwrap_or_else(|| ns_tag.len());
    match ns_tag.split_at(first_colon) {
        (tag, "") => ("", tag),
        (ns, tag) => (ns, &tag[1..]),
    }
}

fn ns_and_tag(name: &QualName) -> (&str, &str) {
    match name.prefix {
        // When parsing with xml5ever, the prefix in `prefix:tag` ends up
        // in the prefix.
        Some(ref prefix) => {
            let ns: &str = prefix;
            let tag: &str = &name.local;
            (ns, tag)
        }
        // When parsing with html5ever, the prefix in `prefix:tag` ends up
        // in the local name and needs to be split out.
        None => {
            let ns_tag: &str = &name.local;
            split_ns_and_tag(ns_tag)
        }
    }
}

impl Encoder for Node {
    fn encode<'a>(&self, env: Env<'a>) -> Term<'a> {
        let struct_atom = atoms::__struct__().encode(env);
        let parent_atom = atoms::parent().encode(env);
        let id_atom = atoms::id().encode(env);

        match self.node {
            Comment(ref content) => {
                let document_comment_atom = atoms::document_comment().encode(env);
                let content_atom = atoms::content().encode(env);
                let keys = vec![struct_atom, parent_atom, id_atom, content_atom];
                let values = vec![
                    document_comment_atom,
                    self.parent.encode(env),
                    self.id.encode(env),
                    STW(content).encode(env),
                ];
                Term::map_from_arrays(env, &keys, &values).ok().unwrap()
            }

            Data(ref data_type, ref content) => {
                let document_data_atom = atoms::document_data().encode(env);
                let type_atom = atoms::type_().encode(env);
                let content_atom = atoms::content().encode(env);
                let keys = vec![struct_atom, parent_atom, id_atom, type_atom, content_atom];
                let values = vec![
                    document_data_atom,
                    self.parent.encode(env),
                    self.id.encode(env),
                    data_type.encode(env),
                    STW(content).encode(env),
                ];
                Term::map_from_arrays(env, &keys, &values).ok().unwrap()
            }

            Doctype(ref name, ref public, ref system) => {
                let document_doctype_atom = atoms::document_doctype().encode(env);
                let name_atom = atoms::name().encode(env);
                let public_atom = atoms::public().encode(env);
                let system_atom = atoms::system().encode(env);
                let keys = vec![
                    struct_atom,
                    parent_atom,
                    id_atom,
                    name_atom,
                    public_atom,
                    system_atom,
                ];
                let values = vec![
                    document_doctype_atom,
                    self.parent.encode(env),
                    self.id.encode(env),
                    STW(name).encode(env),
                    STW(public).encode(env),
                    STW(system).encode(env),
                ];
                Term::map_from_arrays(env, &keys, &values).ok().unwrap()
            }

            Document => unreachable!(),

            Element(ref name, ref attributes, ref _template) => {
                let document_element_atom = atoms::document_element().encode(env);
                let namespace_atom = atoms::namespace().encode(env);
                let tag_atom = atoms::tag().encode(env);
                let attributes_atom = atoms::attributes().encode(env);
                let children_atom = atoms::children().encode(env);
                let (namespace, tag) = ns_and_tag(&name);
                let attribute_terms: Vec<Term<'a>> = attributes
                    .iter()
                    .map(|a| (QNW(&a.name), STW(&a.value)).encode(env))
                    .collect();
                let keys = vec![
                    struct_atom,
                    parent_atom,
                    id_atom,
                    namespace_atom,
                    tag_atom,
                    attributes_atom,
                    children_atom,
                ];
                let values = vec![
                    document_element_atom,
                    self.parent.encode(env),
                    self.id.encode(env),
                    namespace.encode(env),
                    tag.encode(env),
                    attribute_terms.encode(env),
                    self.children.encode(env),
                ];
                Term::map_from_arrays(env, &keys, &values).ok().unwrap()
            }

            ProcessingInstruction(ref target, ref data) => {
                let document_pi_atom = atoms::document_pi().encode(env);
                let target_atom = atoms::target().encode(env);
                let data_atom = atoms::data().encode(env);
                let keys = vec![struct_atom, parent_atom, id_atom, target_atom, data_atom];
                let values = vec![
                    document_pi_atom,
                    self.parent.encode(env),
                    self.id.encode(env),
                    STW(target).encode(env),
                    STW(data).encode(env),
                ];
                Term::map_from_arrays(env, &keys, &values).ok().unwrap()
            }

            Text(ref content) => {
                let document_text_atom = atoms::document_text().encode(env);
                let content_atom = atoms::content().encode(env);
                let keys = vec![struct_atom, parent_atom, id_atom, content_atom];
                let values = vec![
                    document_text_atom,
                    self.parent.encode(env),
                    self.id.encode(env),
                    STW(content).encode(env),
                ];
                Term::map_from_arrays(env, &keys, &values).ok().unwrap()
            }
        }
    }
}

// FlatDom

impl Encoder for FlatDom {
    fn encode<'a>(&self, env: Env<'a>) -> Term<'a> {
        let struct_atom = atoms::__struct__().encode(env);
        let document_atom = atoms::document().encode(env);
        let id_counter_atom = atoms::id_counter().encode(env);
        let roots_atom = atoms::roots().encode(env);
        let nodes_atom = atoms::nodes().encode(env);
        let id_counter = self.nodes.len() - 1;
        let roots = &self.nodes[0].children;
        let (node_keys, node_values): (Vec<_>, Vec<_>) = self
            .nodes
            .iter()
            .skip(1)
            .map(|n| (n.id.encode(env), n.encode(env)))
            .unzip();
        let nodes_term = Term::map_from_arrays(env, &node_keys, &node_values)
            .ok()
            .unwrap();
        let keys = vec![struct_atom, id_counter_atom, roots_atom, nodes_atom];
        let values = vec![
            document_atom,
            id_counter.encode(env),
            roots.encode(env),
            nodes_term,
        ];
        Term::map_from_arrays(env, &keys, &values).ok().unwrap()
    }
}
