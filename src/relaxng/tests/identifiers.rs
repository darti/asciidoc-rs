use ctor::ctor;

use indoc::indoc;
use pretty_env_logger::env_logger::{Builder, Env};

use crate::relaxng::{parser::*, *};
use nom::error::{Error as NomError, ErrorKind, ParseError};

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
