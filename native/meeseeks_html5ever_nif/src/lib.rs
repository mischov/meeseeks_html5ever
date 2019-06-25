#[macro_use]
extern crate rustler;
#[macro_use]
extern crate lazy_static;
extern crate html5ever;
extern crate xml5ever;
#[macro_use]
extern crate markup5ever;
extern crate scoped_pool;
extern crate tendril;

use std::panic;

mod flat_dom;
use flat_dom::FlatDom;

use rustler::{
    Encoder,
    // For use with term_to_configs
    //Decoder,
    //Error,
    Env,
    NifResult,
    Term,
};

use rustler::env::OwnedEnv;
use rustler::types::binary::Binary;

use tendril::TendrilSink;

// If using term_to_configs, remove this mod atoms and use commented

mod atoms {
    rustler_atoms! {
        atom html5ever_nif_result;

        atom ok;
        atom error;
        atom nif_panic;
    }
}

// Not currently using term_to_configs
/*

use html5ever::driver::ParseOpts;
use html5ever::tokenizer::{TokenizerOpts};
use html5ever::tree_builder::TreeBuilderOpts;
use html5ever::tree_builder::interface::QuirksMode;

mod atoms {
    rustler_atoms! {
        atom html5ever_nif_result;

        atom ok;
        atom error;
        atom nil;
        atom nif_panic;

        atom error_level;
        atom discard_bom;
        atom scripting_enabled;
        atom iframe_srcdoc;
        atom drop_doctype;

        atom none;
        atom some;
        atom all;
    }
}

#[derive(PartialEq, Eq)]
enum ErrorLevel {
    None,
    Some,
    All,
}
impl<'a> Decoder<'a> for ErrorLevel {
    fn decode(term: Term<'a>) -> NifResult<ErrorLevel> {
        if atoms::none() == term { Ok(ErrorLevel::None) }
        else if atoms::some() == term { Ok(ErrorLevel::Some) }
        else if atoms::all() == term { Ok(ErrorLevel::All) }
        else { Err(Error::BadArg) }
    }
}

fn term_to_configs(term: Term) -> NifResult<ParseOpts> {
    if atoms::nil() == term {
        Ok(ParseOpts::default())
    } else {
        let env = term.get_env();

        let errors: ErrorLevel =
            term.map_get(atoms::error_level().to_term(env))?.decode()?;

        let discard_bom: bool =
            term.map_get(atoms::discard_bom().to_term(env))?.decode()?;
        let scripting_enabled: bool =
            term.map_get(atoms::scripting_enabled().to_term(env))?.decode()?;
        let iframe_srcdoc: bool =
            term.map_get(atoms::iframe_srcdoc().to_term(env))?.decode()?;
        let drop_doctype: bool =
            term.map_get(atoms::drop_doctype().to_term(env))?.decode()?;

        Ok(ParseOpts {
            tokenizer: TokenizerOpts {
                exact_errors: errors == ErrorLevel::All,
                discard_bom: discard_bom,
                profile: false,
                initial_state: None,
                last_start_tag_name: None,
            },
            tree_builder: TreeBuilderOpts {
                exact_errors: errors == ErrorLevel::All,
                scripting_enabled: scripting_enabled,
                iframe_srcdoc: iframe_srcdoc,
                drop_doctype: drop_doctype,
                ignore_missing_rules: false,
                quirks_mode: QuirksMode::NoQuirks,
            },
        })
    }
}
*/

enum ParserType {
    HtmlDocument,
    XmlDocument,
}

// Thread pool for `parse_async`.
// TODO: How do we decide on pool size?
lazy_static! {
    static ref POOL: scoped_pool::Pool = scoped_pool::Pool::new(4);
}

fn parse<'a>(parser_type: ParserType, env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    let mut owned_env = OwnedEnv::new();

    // Copies the term into the inner env. Since this term is normally a large
    // binary term, copying it over should be cheap, since the binary will be
    // refcounted within the BEAM.
    let input_term = owned_env.save(args[0]);

    let return_pid = env.pid();

    //let config = term_to_configs(args[1]);

    POOL.spawn(move || {
        owned_env.send_and_clear(&return_pid, |inner_env| {
            // This should not really be done in user code. We (Rustler project)
            // need to find a better abstraction that eliminates this.
            match panic::catch_unwind(|| {
                let binary: Binary = match input_term.load(inner_env).decode() {
                    Ok(inner) => inner,
                    Err(_) => panic!("argument is not a binary"),
                };

                let sink = FlatDom::default();

                let result = match parser_type {
                    ParserType::HtmlDocument => {
                        // TODO: Use Parser.from_bytes instead?
                        let parser = html5ever::parse_document(sink, Default::default());

                        match std::str::from_utf8(binary.as_slice()) {
                            Ok(decoded) => parser.one(decoded),
                            Err(_) => panic!("input is not valid utf8"),
                        }
                    }

                    ParserType::XmlDocument => {
                        // TODO: Use Parser.from_bytes instead?
                        let parser = xml5ever::driver::parse_document(sink, Default::default());

                        match std::str::from_utf8(binary.as_slice()) {
                            Ok(decoded) => parser.one(decoded),
                            Err(_) => panic!("input is not valid utf8"),
                        }

                    }
                };

                let result_term = result.encode(inner_env);

                //let result_term = handle_to_term(inner_env, &index, &Parent::None, &result.document);

                (atoms::html5ever_nif_result(), atoms::ok(), result_term).encode(inner_env)
            }) {
                Ok(term) => term,
                Err(err) => {
                    // Try to extract a panic reason and return that. If this
                    // fails, fail generically.
                    let reason = if let Some(s) = err.downcast_ref::<String>() {
                        s.encode(inner_env)
                    } else if let Some(&s) = err.downcast_ref::<&'static str>() {
                        s.encode(inner_env)
                    } else {
                        atoms::nif_panic().encode(inner_env)
                    };
                    (atoms::html5ever_nif_result(), atoms::error(), reason).encode(inner_env)
                }
            }
        });
    });

    Ok(atoms::ok().encode(env))
}

fn parse_html<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    parse(ParserType::HtmlDocument, env, args)
}

fn parse_xml<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    parse(ParserType::XmlDocument, env, args)
}

rustler_export_nifs!(
    "Elixir.MeeseeksHtml5ever.Native",
    [("parse_html", 1, parse_html), ("parse_xml", 1, parse_xml),],
    Some(on_load)
);

fn on_load<'a>(_env: Env<'a>, _load_info: Term<'a>) -> bool {
    true
}
