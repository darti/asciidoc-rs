use nom::{
    bytes::complete::tag,
    character::complete::{alphanumeric1, char, multispace0},
    multi::many0,
    sequence::terminated,
    IResult,
};

use crate::relaxng::{parser::*, *};

use super::{
    common::quoted,
    namespaces::{default_namespace, namespace},
};

pub(crate) fn datatypes(input: Span) -> IResult<Span, Decl> {
    let (input, _) = terminated(tag("datatypes"), multispace0)(input)?;

    let (input, id) = terminated(alphanumeric1, multispace0)(input)?;
    let (input, _) = terminated(char('='), multispace0)(input)?;
    let (input, ns) = terminated(quoted, multispace0)(input)?;

    Ok((input, Decl::Datatypes(id.to_string(), ns.to_string())))
}

pub(crate) fn decls(input: Span) -> IResult<Span, Vec<Decl>> {
    many0(alt((namespace, default_namespace, datatypes)))(input)
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use crate::relaxng::{
        parser::{
            declarations::{datatypes, decls},
            Span,
        },
        Decl,
    };

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
}
