use asciidoc_rs::relaxng::*;
use ctor::ctor;

use log::info;
use pretty_env_logger::env_logger::{Builder, Env};
use serde_roxmltree::from_doc;

#[ctor]
fn init_logger() {
    Builder::from_env(Env::new().default_filter_or("info"))
        .is_test(true)
        .init();
}

#[test]
fn test_docbook() {
    let src = include_str!("../schemas/docbook.rng.xml");

    let doc = roxmltree::Document::parse(src).unwrap();

    // from_doc(&doc);

    // info!("{:?}", doc);
}

#[test]
fn test_spec_3_1() {
    let src = include_str!("./schemas/relaxng_3_1.rng.xml");

    let doc = roxmltree::Document::parse(src).unwrap();

    // from_doc(&doc);

    // info!("{:?}", doc);
}
