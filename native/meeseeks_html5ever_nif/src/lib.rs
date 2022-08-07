use rustler::types::binary::Binary;
use rustler::{Decoder, Encoder, Env, Error, NifResult, Term};

//use html5ever::rcdom::RcDom;
use tendril::TendrilSink;

mod common;
mod flat_dom;

mod atoms {
    rustler::atoms! {
        html5ever_nif_result,

        ok,
        error,
        nif_panic,

        doctype,
        comment,

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
        if atoms::none() == term {
            Ok(ErrorLevel::None)
        } else if atoms::some() == term {
            Ok(ErrorLevel::Some)
        } else if atoms::all() == term {
            Ok(ErrorLevel::All)
        } else {
            Err(Error::BadArg)
        }
    }
}

enum ParserType {
    HtmlDocument,
    XmlDocument,
}

fn parse<'a>(parser_type: ParserType, env: Env<'a>, document: Binary) -> Term<'a> {
    let sink = flat_dom::FlatSink::new();

    let decoded = std::str::from_utf8(document.as_slice()).unwrap();

    let result = match parser_type {
        ParserType::HtmlDocument => {
            let parser = html5ever::parse_document(sink, Default::default());
            parser.one(decoded)
        }
        ParserType::XmlDocument => {
            let parser = xml5ever::driver::parse_document(sink, Default::default());
            parser.one(decoded)
        }
    };

    let result_term = flat_dom::flat_sink_to_flat_term(env, &result);
    (atoms::html5ever_nif_result(), atoms::ok(), result_term).encode(env)
}

#[rustler::nif(schedule = "DirtyCpu")]
fn parse_html<'a>(env: Env<'a>, document: Binary) -> Term<'a> {
    parse(ParserType::HtmlDocument, env, document)
}

#[rustler::nif(schedule = "DirtyCpu")]
fn parse_xml<'a>(env: Env<'a>, document: Binary) -> Term<'a> {
    parse(ParserType::XmlDocument, env, document)
}

rustler::init!(
    "Elixir.MeeseeksHtml5ever.Native",
    [parse_html, parse_xml],
    load = on_load
);

fn on_load<'a>(_env: Env<'a>, _load_info: Term<'a>) -> bool {
    true
}
