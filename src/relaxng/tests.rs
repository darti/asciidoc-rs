use ctor::ctor;
use indoc::indoc;

use log::info;
use pretty_env_logger::env_logger::{Builder, Env};
use serde_roxmltree::from_doc;

use super::*;

#[ctor]
fn init_logger() {
    Builder::from_env(Env::new().default_filter_or("info"))
        .is_test(true)
        .init();
}

#[test]
fn test_empty_schema() -> anyhow::Result<()> {
    let src = indoc! {r#"<grammar xmlns="http://relaxng.org/ns/structure/1.0"></grammar>"#};

    let doc = roxmltree::Document::parse(src)?;
    let schema: Grammar = from_doc(&doc)?;

    let should_be = Grammar {
        grammar_content: vec![],
    };

    assert_eq!(schema, should_be);

    Ok(())
}

#[test]
fn test_start() -> anyhow::Result<()> {
    let src = indoc! {r#"

        <start></start>

    "#};

    let doc = roxmltree::Document::parse(src)?;
    let schema: GrammarContent = from_doc(&doc)?;

    info!("{:?}", schema);

    let should_be = Grammar {
        grammar_content: vec![GrammarContent::Start(Start {
            pattern: Pattern::Empty,
        })],
    };

    // assert_eq!(schema, should_be);
    Ok(())
}

// #[test]
// fn test_grammar() {
//     let src = indoc! {r#"
//         <grammar xmlns="http://relaxng.org/ns/structure/1.0">
//         <start></start>
//         </grammar>
//     "#};

//     let doc = roxmltree::Document::parse(src).unwrap();
//     let schema: Result<Grammar, _> = from_doc(&doc);

//     info!("{:?}", doc);

//     let should_be = Grammar {
//         grammar_content: vec![GrammarContent::Start(Start {})],
//     };

//     assert_eq!(schema.unwrap(), should_be);
// }
