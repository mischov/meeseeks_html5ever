#[macro_use]
extern crate rustler;
extern crate html5ever;
extern crate xml5ever;
#[macro_use]
extern crate markup5ever;
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

use rustler::types::binary::Binary;

use tendril::TendrilSink;

// If using term_to_configs, remove this mod atoms and use commented

mod atoms {
    atoms! {
        html5ever_nif_result,

        ok,
        error,
        nif_panic,
    }
}

// Not currently using term_to_configs
/*
use html5ever::driver::ParseOpts;
use html5ever::tokenizer::{TokenizerOpts};
use html5ever::tree_builder::TreeBuilderOpts;
use html5ever::tree_builder::interface::QuirksMode;
mod atoms {
    atoms! {
        html5ever_nif_result,
        ok,
        error,
        nil,
        nif_panic,
        error_level,
        discard_bom,
        scripting_enabled,
        iframe_srcdoc,
        drop_doctype,
        none,
        some,
        all,
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

fn parse<'a>(parser_type: ParserType, env: Env<'a>, document: Binary) -> NifResult<Term<'a>> {
    match panic::catch_unwind(|| {
        let sink = FlatDom::default();

        let result = match parser_type {
            ParserType::HtmlDocument => {
                let parser = html5ever::parse_document(sink, Default::default());

                match std::str::from_utf8(document.as_slice()) {
                    Ok(decoded) => parser.one(decoded),
                    Err(_) => panic!("input is not valid utf8"),
                }
            }

            ParserType::XmlDocument => {
                let parser = xml5ever::driver::parse_document(sink, Default::default());

                match std::str::from_utf8(document.as_slice()) {
                    Ok(decoded) => parser.one(decoded),
                    Err(_) => panic!("input is not valid utf8"),
                }
            }
        };

        let result_term = result.encode(env);

        (atoms::html5ever_nif_result(), atoms::ok(), result_term).encode(env)
    }) {
        Ok(term) => Ok(term),
        Err(err) => {
            // Try to extract a panic reason and return that. If this
            // fails, fail generically.
            let reason = if let Some(s) = err.downcast_ref::<String>() {
                s.encode(env)
            } else if let Some(&s) = err.downcast_ref::<&'static str>() {
                s.encode(env)
            } else {
                atoms::nif_panic().encode(env)
            };
            Ok((atoms::html5ever_nif_result(), atoms::error(), reason).encode(env))
        }
    }
}

#[rustler::nif(schedule = "DirtyCpu")]
fn parse_html<'a>(env: Env<'a>, document: Binary) -> NifResult<Term<'a>> {
    parse(ParserType::HtmlDocument, env, document)
}

#[rustler::nif(schedule = "DirtyCpu")]
fn parse_xml<'a>(env: Env<'a>, document: Binary) -> NifResult<Term<'a>> {
    parse(ParserType::XmlDocument, env, document)
}

rustler::init!("Elixir.MeeseeksHtml5ever.Native", [parse_html, parse_xml], load = load);

fn load(_env: Env, _load_info: Term) -> bool {
    true
}
