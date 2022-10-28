use asciidoc_rs::relaxng::parse;
use ctor::ctor;

use log::info;
use pretty_env_logger::env_logger::{Builder, Env};

#[ctor]
fn init_logger() {
    Builder::from_env(Env::new().default_filter_or("info"))
        .is_test(true)
        .init();
}

#[test]
fn test_docbook() {
    let s = include_str!("docbook.rnc");

    let r = parse(s);

    info!("{:?}", r);
}
