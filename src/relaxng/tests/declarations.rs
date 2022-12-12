use ctor::ctor;

use indoc::indoc;
use pretty_env_logger::env_logger::{Builder, Env};

use crate::relaxng::{parser::*, *};
use nom::error::{Error as NomError, ErrorKind, ParseError};

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
