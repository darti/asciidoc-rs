use ctor::ctor;
use indoc::indoc;

use log::info;
use pretty_env_logger::env_logger::{Builder, Env};
use serde_xml_rs::{from_str, to_string};

use super::*;

#[ctor]
fn init_logger() {
    Builder::from_env(Env::new().default_filter_or("info"))
        .is_test(true)
        .init();
}

#[test]
fn test_empty_schema() {
    let src = indoc! {r#"
        <grammar xmlns="http://relaxng.org/ns/structure/1.0"></grammar>
    "#};

    let grammar: Result<Grammar, _> = from_str(src);
    let should_be = GrammarBuilder::default().build().unwrap();

    assert_eq!(grammar.unwrap(), should_be);
}

#[test]
fn write_schema() {
    let grammar = GrammarBuilder::default()
        .content(GrammarContent::Start(
            StartBuilder::default().build().unwrap(),
        ))
        .build()
        .unwrap();

    let src = Pattern::Grammar(grammar);

    let output = to_string(&src).unwrap();

    info!("{}", output)
}
