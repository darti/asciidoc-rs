use asciidoc_rs::relaxng::*;
use ctor::ctor;

use log::info;
use pretty_env_logger::env_logger::{Builder, Env};
use serde_xml_rs::{from_str, to_string};

#[ctor]
fn init_logger() {
    Builder::from_env(Env::new().default_filter_or("info"))
        .is_test(true)
        .init();
}

#[test]
fn test_docbook() {
    let src = include_str!("../schemas/docbook.rng.xml");

    let grammar: Pattern = from_str(src).unwrap();

    info!("{:?}", grammar);
}
