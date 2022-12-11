use ctor::ctor;

use indoc::indoc;
use pretty_env_logger::env_logger::{Builder, Env};

use crate::relaxng::{parser::*, *};
use nom::error::{Error as NomError, ErrorKind, ParseError};

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
fn test_identifier_simple() {
    let i = Span::new("maurice");

    let (_, o) = identifier(i).unwrap();

    assert_eq!(o.to_string(), "maurice");
}

#[test]
fn test_identifier_with_dots() {
    let i = Span::new("good_id.666");

    let (_, o) = identifier(i).unwrap();

    assert_eq!(o.to_string(), "good_id.666");
}

#[test]
fn test_identifier_keyword() {
    let i = Span::new("attribute");

    let r = identifier(i);

    assert_eq!(
        r,
        Err(nom::Err::Error(NomError::from_error_kind(
            i,
            ErrorKind::RegexpCapture
        ),))
    );
}

#[test]
fn test_identifier_or_keyword_identifier() {
    let i = Span::new("good_id.666");

    let (_, o) = identifier_or_keyword(i).unwrap();

    assert_eq!(o.to_string(), "good_id.666");
}

#[test]
fn test_identifier_or_keyword_keyword() {
    let i = Span::new("attribute");

    let (_, o) = identifier_or_keyword(i).unwrap();

    assert_eq!(o.to_string(), "attribute");
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

#[test]
fn test_datatypes() {
    let i = Span::new("datatypes xsd = \"http://www.w3.org/2001/XMLSchema-datatypes\"");

    let (_, o) = datatypes(i).unwrap();

    assert_eq!(
        o,
        Decl::Datatypes(
            "xsd".into(),
            "http://www.w3.org/2001/XMLSchema-datatypes".into()
        )
    );
}

#[test]
fn test_decls() {
    let i = Span::new(indoc! {r#"
        default namespace rng = "http://relaxng.org/ns/structure/1.0"
        namespace local = ""
        datatypes xsd = "http://www.w3.org/2001/XMLSchema-datatypes"
    "#});

    let (_, o) = decls(i).unwrap();

    assert_eq!(
        o,
        vec![
            Decl::DefaultNamespace(
                Some("rng".into()),
                "http://relaxng.org/ns/structure/1.0".into()
            ),
            Decl::Namespace("local".into(), "".into()),
            Decl::Datatypes(
                "xsd".into(),
                "http://www.w3.org/2001/XMLSchema-datatypes".into()
            )
        ]
    );
}

// #[test]
// fn test_start() {
//     let i = Span::new(indoc! {r#"
//         start = pattern
//     "#});

//     let (_, o) = start(i).unwrap();

//     assert_eq!(
//         o,
//         GrammarContent::Start(AssignMethod::Assign, Pattern::Identifier("pattern".into()))
//     );
// }
