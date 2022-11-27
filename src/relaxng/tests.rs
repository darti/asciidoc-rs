use ctor::ctor;

use pretty_env_logger::env_logger::{Builder, Env};

use crate::relaxng::{parser::*, Decl};

#[ctor]
fn init_logger() {
    Builder::from_env(Env::new().default_filter_or("info"))
        .is_test(true)
        .init();
}

#[test]
fn test_quoted() {
    let i = Span::new("\"http://relaxng.org/ns/compatibility/annotations/1.0\"");

    let (_, o) = quoted(i).unwrap();

    assert_eq!(
        o.to_string(),
        "http://relaxng.org/ns/compatibility/annotations/1.0"
    );
}

#[test]
fn test_namespace() {
    let i = Span::new("namespace a = \"http://relaxng.org/ns/compatibility/annotations/1.0\"");

    let (_, o) = namespace(i).unwrap();

    assert_eq!(
        o,
        Decl::Namespace(
            "a".into(),
            "http://relaxng.org/ns/compatibility/annotations/1.0".into()
        )
    );
}
