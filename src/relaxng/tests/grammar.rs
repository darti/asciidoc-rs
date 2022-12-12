use ctor::ctor;

use indoc::indoc;
use pretty_env_logger::env_logger::{Builder, Env};

use crate::relaxng::{parser::*, *};
use nom::error::{Error as NomError, ErrorKind, ParseError};

#[test]
fn test_start() {
    let i = Span::new(indoc! {r#"
        start = pattern
    "#});

    let (_, o) = start(i).unwrap();

    assert_eq!(
        o,
        GrammarContent::Start(AssignMethod::Assign, Pattern::Identifier("pattern".into()))
    );
}
