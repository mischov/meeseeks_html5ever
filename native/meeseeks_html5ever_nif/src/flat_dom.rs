use std::borrow::Cow;
use std::collections::HashSet;
use std::default::Default;

use html5ever::{ QualName };
use html5ever::tokenizer::{Attribute};
use html5ever::tree_builder::{TreeSink, QuirksMode, NodeOrText, AppendNode, AppendText};
use tendril::StrTendril;

use rustler::{NifEnv, NifTerm, NifEncoder};
use rustler::types::elixir_struct::{ make_ex_struct};
use rustler::types::map::{ map_new };

use self::NodeEnum::{Comment, Data, Doctype, Document, Element, Text};

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
enum NodeEnum {
    Comment(StrTendril),
    Data(StrTendril),
    Doctype(StrTendril, StrTendril, StrTendril),
    Document,
    Element(QualName, Vec<Attribute>),
    Text(StrTendril),
}

impl NodeEnum {
    fn script_or_style(&self) -> bool {
        match *self {
            Element(ref name, _) => {
                match *name {
                    qualname!(html, "script") | qualname!(html, "style") =>
                        true,
                    _ => false,
                }
            },
            _ => false,
        }
    }

    fn append_text(&mut self, text: &str) -> bool {
        match *self {
            Text(ref mut current) | Data(ref mut current) => {
                current.push_slice(text);
                true
            },
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
            children: vec!(),
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
    nodes: Vec<Node>
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
            let child = if self.node(parent).node.script_or_style() {
                self.add_node(Data(text))} else {
                self.add_node(Text(text))
            };
            self.node_mut(child).parent = Parent::Some(parent);
            let parent_node = self.node_mut(parent);
            parent_node.children.push(child);
            parent_node.last_string = true;
        }
    }

    fn get_parent_and_index(&self, child: Id) -> (Id, usize) {
        let maybe_parent = &self.node(child).parent;
        match *maybe_parent {
            Parent::None => panic!("expected parent found none"),
            Parent::Some(parent) => {
                match self.node(parent).index_of_child(child) {
                    Some(i) => (parent, i),
                    None => panic!("have parent but not in parent"),
                }
            }
        }
    }

    fn remove_from_parent(&mut self, child: Id) {
        let (parent, i) = self.get_parent_and_index(child);
        self.node_mut(parent).children.remove(i);
        let child = self.node_mut(child);
        child.parent = Parent::None;
    }
}

impl Default for FlatDom {
    fn default() -> FlatDom {
        FlatDom {
            nodes: vec![Node::new(Id(0), Document)]
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

    // Not supported
    fn get_template_contents(&mut self, _target: Self::Handle) -> Self::Handle {
        panic!("Templates not supported");
    }

    // Not supported
    fn set_quirks_mode(&mut self, _mode: QuirksMode) {}

    fn same_node(&self, x: Self::Handle, y: Self::Handle) -> bool {
        x == y
    }

    fn elem_name(&self, target: Self::Handle) -> QualName {
        if let Element(ref name, _) = self.node(target).node {
            name.clone()
        } else {
            panic!("not an element!")
        }
    }

    fn create_element(&mut self, name: QualName, attrs: Vec<Attribute>) -> Self::Handle {
        self.add_node(Element(name, attrs))
    }

    fn create_comment(&mut self, text: StrTendril) -> Self::Handle {
        self.add_node(Comment(text))
    }

    fn has_parent_node(&self, node: Self::Handle) -> bool {
        match self.node(node).parent {
            Parent::None => false,
            _ => true,
        }
    }

    fn append(&mut self, parent: Self::Handle, child: NodeOrText<Self::Handle>) {
        match child {
            AppendNode(node) => self.append_node(parent, node),
            AppendText(text) => self.append_text(parent, text),
        };
    }

    fn append_before_sibling(&mut self, sibling: Self::Handle, child: NodeOrText<Self::Handle>) {
        let (parent, i) = self.get_parent_and_index(sibling);

        let child = match (child, i) {
            // No previous node
            (AppendText(text), 0) => self.add_node(Text(text)),

            // Check for text node before insertion point, append if there is
            (AppendText(text), i) => {
                let prev = self.node(parent).children[i-1];
                if self.node_mut(prev).node.append_text(&text) {
                    return;
                }
                self.add_node(Text(text))
            },

            // Tree builder promises no text no *after* insertion point

            // Any other kind of node
            (AppendNode(node), _) => node,
        };

        if self.node(child).parent.is_some() {
            self.remove_from_parent(child);
        }

        self.node_mut(child).parent = Parent::Some(parent);
        self.node_mut(parent).children.insert(i, child);
    }

    fn append_doctype_to_document(&mut self, name: StrTendril, public_id: StrTendril, system_id: StrTendril) {
        let doctype = self.add_node(Doctype(name, public_id, system_id));
        self.append_node(Id(0), doctype);
    }

    fn add_attrs_if_missing(&mut self, target: Self::Handle, attrs: Vec<Attribute>) {
        let target_node = self.node_mut(target);
        let target_attrs = if let Element(_, ref mut attrs) = target_node.node {
            attrs
        } else {
            panic!("not an element")
        };

        let existing_names = target_attrs.iter().map(|e| e.name.clone())
            .collect::<HashSet<_>>();
        target_attrs.extend(attrs.into_iter().filter(|attr| {
            !existing_names.contains(&attr.name)
        }));
    }

    fn remove_from_parent(&mut self, target: Self::Handle) {
        self.remove_from_parent(target);
    }

    fn reparent_children(&mut self, node: Self::Handle, new_parent: Self::Handle) {
        let children = self.node(node).children.clone();
        for child in &children {
            self.remove_from_parent(*child);
            self.append_node(new_parent, *child);
        }
    }

    // Not supported
    fn mark_script_already_started(&mut self, _target: Self::Handle) {
        panic!("not supported")
    }
}

// NIF Encoding

mod atoms {
    rustler_atoms! {
        atom nil;

        atom parent;
        atom id;
        atom content;
        atom name;
        atom public;
        atom system;
        atom namespace;
        atom tag;
        atom attributes;
        atom children;

        atom id_counter;
        atom roots;
        atom nodes;
    }
}

// QualName and StrTendril

// Zero-cost wrapper types which makes it possible to implement
// NifEncoder for these externally defined types.
// Unsure if this is a great way of doing it, but it's the way
// that produced the cleanest and least noisy code.

struct QNW<'a>(&'a QualName);

impl<'b> NifEncoder for QNW<'b> {
    fn encode<'a>(&self, env: NifEnv<'a>) -> NifTerm<'a> {
        let local: &str = &*self.0.local;
        local.encode(env)
    }
}

struct STW<'a>(&'a StrTendril);

impl<'b> NifEncoder for STW<'b> {
    fn encode<'a>(&self, env: NifEnv<'a>) -> NifTerm<'a> {
        let data: &str = &*self.0;
        data.encode(env)
    }
}

// Id

impl NifEncoder for Id {
    fn encode<'a>(&self, env: NifEnv<'a>) -> NifTerm<'a> {
        self.0.encode(env)
    }
}

// Parent

impl NifEncoder for Parent {
    fn encode<'a>(&self, env: NifEnv<'a>) -> NifTerm<'a> {
        match *self {
            Parent::None => atoms::nil().encode(env),
            Parent::Some(id) => if id == Id(0) {
                atoms::nil().encode(env)
            } else {
                id.encode(env)}
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

impl NifEncoder for Node {
    fn encode<'a>(&self, env: NifEnv<'a>) -> NifTerm<'a> {
        let parent_atom = atoms::parent().encode(env);
        let id_atom = atoms::id().encode(env);

        match self.node {
            Comment(ref content) => {
                let content_atom = atoms::content().encode(env);
                make_ex_struct(env, "Elixir.Meeseeks.Document.Comment").ok().unwrap()
                    .map_put(parent_atom, self.parent.encode(env)).ok().unwrap()
                    .map_put(id_atom, self.id.encode(env)).ok().unwrap()
                    .map_put(content_atom, STW(content).encode(env)).ok().unwrap()
            },

            Data(ref content) => {
                let content_atom = atoms::content().encode(env);
                make_ex_struct(env, "Elixir.Meeseeks.Document.Data").ok().unwrap()
                    .map_put(parent_atom, self.parent.encode(env)).ok().unwrap()
                    .map_put(id_atom, self.id.encode(env)).ok().unwrap()
                    .map_put(content_atom, STW(content).encode(env)).ok().unwrap()
            },

            Doctype(ref name, ref public, ref system) => {
                let name_atom = atoms::name().encode(env);
                let public_atom = atoms::public().encode(env);
                let system_atom = atoms::system().encode(env);
                make_ex_struct(env, "Elixir.Meeseeks.Document.Doctype").ok().unwrap()
                    .map_put(parent_atom, self.parent.encode(env)).ok().unwrap()
                    .map_put(id_atom, self.id.encode(env)).ok().unwrap()
                    .map_put(name_atom, STW(name).encode(env)).ok().unwrap()
                    .map_put(public_atom, STW(public).encode(env)).ok().unwrap()
                    .map_put(system_atom, STW(system).encode(env)).ok().unwrap()
            },

            Document => unreachable!(),

            Element(ref name, ref attributes) => {
                let namespace_atom = atoms::namespace().encode(env);
                let tag_atom = atoms::tag().encode(env);
                let attributes_atom = atoms::attributes().encode(env);
                let children_atom = atoms::children().encode(env);
                let ns_tag: &str = &name.local;
                let (namespace, tag) = split_ns_and_tag(ns_tag);
                let attribute_terms: Vec<NifTerm<'a>> =
                    attributes.iter()
                    .map(|a| (QNW(&a.name), STW(&a.value)).encode(env))
                    .collect();
                make_ex_struct(env, "Elixir.Meeseeks.Document.Element").ok().unwrap()
                    .map_put(parent_atom, self.parent.encode(env)).ok().unwrap()
                    .map_put(id_atom, self.id.encode(env)).ok().unwrap()
                    .map_put(namespace_atom, namespace.encode(env)).ok().unwrap()
                    .map_put(tag_atom, tag.encode(env)).ok().unwrap()
                    .map_put(attributes_atom, attribute_terms.encode(env)).ok().unwrap()
                    .map_put(children_atom, self.children.encode(env)).ok().unwrap()
            },

            Text(ref content) => {
                let content_atom = atoms::content().encode(env);
                make_ex_struct(env, "Elixir.Meeseeks.Document.Text").ok().unwrap()
                    .map_put(parent_atom, self.parent.encode(env)).ok().unwrap()
                    .map_put(id_atom, self.id.encode(env)).ok().unwrap()
                    .map_put(content_atom, STW(content).encode(env)).ok().unwrap()
            },
        }
    }
}

// FlatDom

impl NifEncoder for FlatDom {
    fn encode<'a>(&self, env: NifEnv<'a>) -> NifTerm<'a> {
        let id_counter_atom = atoms::id_counter().encode(env);
        let roots_atom = atoms::roots().encode(env);
        let nodes_atom = atoms::nodes().encode(env);
        let id_counter = self.nodes.len() - 1;
        let roots = &self.nodes[0].children;
        let nodes = map_new(env);
        let nodes_term = self.nodes.iter().skip(1).fold(nodes, |m, n|
                                                      m.map_put(n.id.encode(env), n.encode(env)).ok().unwrap());
        make_ex_struct(env, "Elixir.Meeseeks.Document").ok().unwrap()
            .map_put(id_counter_atom, id_counter.encode(env)).ok().unwrap()
            .map_put(roots_atom, roots.encode(env)).ok().unwrap()
            .map_put(nodes_atom, nodes_term).ok().unwrap()
    }
}
