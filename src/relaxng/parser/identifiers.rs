use nom::{
    branch::alt,
    bytes::complete::tag,
    error::{Error as NomError, ErrorKind, ParseError},
    IResult, Slice,
};

use lazy_static::lazy_static;
use regex::Regex;

use super::Span;

const KEYWORDS: [&'static str; 19] = [
    "attribute",
    "default",
    "datatypes",
    "div",
    "element",
    "empty",
    "external",
    "grammar",
    "include",
    "inherit",
    "list",
    "mixed",
    "namespace",
    "notAllowed",
    "parent",
    "start",
    "string",
    "text",
    "token",
];

pub(crate) fn identifier_or_keyword(input: Span) -> IResult<Span, Span> {
    alt((identifier, keyword))(input)
}

pub(crate) fn identifier(input: Span) -> IResult<Span, Span> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"[_\p{XID_Start}][\._\p{XID_Continue}]*").unwrap();
    }

    match RE.find(&input) {
        Some(id) if KEYWORDS.contains(&id.as_str()) => Err(nom::Err::Error(
            NomError::from_error_kind(input.clone(), ErrorKind::RegexpCapture),
        )),
        Some(id) => Ok((input.slice(id.end()..), id.as_str().into())),
        None => todo!(),
    }
}

pub(crate) fn keyword(input: Span) -> IResult<Span, Span> {
    alt((
        tag("attribute"),
        tag("default"),
        tag("datatypes"),
        tag("div"),
        tag("element"),
        tag("empty"),
        tag("external"),
        tag("grammar"),
        tag("include"),
        tag("inherit"),
        tag("list"),
        tag("mixed"),
        tag("namespace"),
        tag("notAllowed"),
        tag("parent"),
        tag("start"),
        tag("string"),
        tag("text"),
        tag("token"),
    ))(input)
}

#[cfg(test)]
mod tests {
    use crate::relaxng::parser::{
        common::quoted,
        identifiers::{identifier, identifier_or_keyword},
        Span,
    };

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
}
