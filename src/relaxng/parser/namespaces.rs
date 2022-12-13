use crate::relaxng::{parser::*, *};
use nom::{
    bytes::complete::tag,
    character::complete::{alphanumeric1, char, multispace0},
    combinator::opt,
    sequence::terminated,
    IResult,
};

use super::common::quoted;

pub(crate) fn namespace(input: Span) -> IResult<Span, Decl> {
    let (input, _) = terminated(tag("namespace"), multispace0)(input)?;

    let (input, id) = terminated(alphanumeric1, multispace0)(input)?;
    let (input, _) = terminated(char('='), multispace0)(input)?;
    let (input, ns) = terminated(quoted, multispace0)(input)?;

    Ok((input, Decl::Namespace(id.to_string(), ns.to_string())))
}

pub(crate) fn default_namespace(input: Span) -> IResult<Span, Decl> {
    let (input, _) = terminated(tag("default"), multispace0)(input)?;
    let (input, _) = terminated(tag("namespace"), multispace0)(input)?;

    let (input, id) = opt(terminated(alphanumeric1, multispace0))(input)?;
    let (input, _) = terminated(char('='), multispace0)(input)?;
    let (input, ns) = terminated(quoted, multispace0)(input)?;

    Ok((
        input,
        Decl::DefaultNamespace(id.map(|i| i.to_string()), ns.to_string()),
    ))
}

#[cfg(test)]
mod tests {
    use crate::relaxng::{
        parser::{
            namespaces::{default_namespace, namespace},
            Span,
        },
        Decl,
    };

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
        let i = Span::new(
            "default namespace = \"http://relaxng.org/ns/compatibility/annotations/1.0\"",
        );

        let (_, o) = default_namespace(i).unwrap();

        assert_eq!(
            o,
            Decl::DefaultNamespace(
                None,
                "http://relaxng.org/ns/compatibility/annotations/1.0".into()
            )
        );
    }
}
