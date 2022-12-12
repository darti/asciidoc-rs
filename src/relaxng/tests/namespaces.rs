use ctor::ctor;

use indoc::indoc;
use pretty_env_logger::env_logger::{Builder, Env};

use crate::relaxng::{parser::*, *};
use nom::error::{Error as NomError, ErrorKind, ParseError};

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

#[test]
fn test_default_namespace() {
    let i = Span::new("default namespace rng = \"http://relaxng.org/ns/structure/1.0\"");

    let (_, o) = default_namespace(i).unwrap();

    assert_eq!(
        o,
        Decl::DefaultNamespace(
            Some("rng".into()),
            "http://relaxng.org/ns/structure/1.0".into()
        )
    );
}

#[test]
fn test_default_namespace_omit_id() {
    let i =
        Span::new("default namespace = \"http://relaxng.org/ns/compatibility/annotations/1.0\"");

    let (_, o) = default_namespace(i).unwrap();

    assert_eq!(
        o,
        Decl::DefaultNamespace(
            None,
            "http://relaxng.org/ns/compatibility/annotations/1.0".into()
        )
    );
}
