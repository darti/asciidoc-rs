use nom::bytes::complete::{escaped, take_until};
use nom::character::complete::{multispace0, multispace1, newline, one_of};
use nom::error::Error as NomError;
use nom::multi::{many0, separated_list0};
use nom::{
    bytes::complete::tag,
    bytes::complete::tag_no_case,
    character::complete::{alphanumeric1, char},
    combinator::map_res,
    sequence::{delimited, separated_pair},
    IResult,
};

use nom_locate::LocatedSpan;
use url::Url;

use super::Namespace;

type Span<'a> = LocatedSpan<&'a str>;

/// A combinator that takes a parser `inner` and produces a parser that also consumes both leading and
/// trailing whitespace, returning the output of `inner`.
fn ws<'a, F, O, E>(inner: F) -> impl FnMut(Span<'a>) -> IResult<Span<'a>, O, E>
where
    F: Fn(Span<'a>) -> IResult<Span<'a>, O, E>,
    E: nom::error::ParseError<Span<'a>>,
{
    delimited(multispace0, inner, multispace0)
}

fn quoted(input: Span) -> IResult<Span, Span> {
    delimited(
        char('"'),
        escaped(take_until("\""), '\\', one_of(r#""n\"#)),
        char('"'),
    )(input)
}

fn parse_namespace(input: Span) -> IResult<Span, Namespace> {
    let (input, _) = tag_no_case("namespace")(input)?;
    let (input, _) = multispace1(input)?;

    map_res(
        separated_pair(alphanumeric1, ws(tag("=")), quoted),
        |(k, v): (Span, Span)| {
            Ok::<Namespace, NomError<Span>>(Namespace {
                name: k.to_string(),
                url: Url::parse(&v).unwrap(),
            })
        },
    )(input)
}
fn parse_namespaces(input: Span) -> IResult<Span, Vec<Namespace>> {
    separated_list0(newline, parse_namespace)(input)
}

pub fn parse(s: &str) {
    let span = Span::new(s);

    // SchemaBuilder::default().build();
}

#[cfg(test)]
mod tests {
    use super::super::NamespaceBuilder;
    use super::*;
    use indoc::indoc;
    use url::Url;

    #[test]
    fn test_ws_none() {
        let i = Span::new("test");

        let (_, o) = ws(tag::<&str, LocatedSpan<&str>, ()>("test"))(i).unwrap();

        assert_eq!(o.to_string(), "test");
    }

    #[test]
    fn test_ws_left() {
        let i = Span::new(" test");

        let (_, o) = ws(tag::<&str, LocatedSpan<&str>, ()>("test"))(i).unwrap();

        assert_eq!(o.to_string(), "test");
    }

    #[test]
    fn test_ws_right() {
        let i = Span::new("test ");

        let (_, o) = ws(tag::<&str, LocatedSpan<&str>, ()>("test"))(i).unwrap();

        assert_eq!(o.to_string(), "test");
    }

    #[test]
    fn test_ws_both() {
        let i = Span::new(" test ");

        let (_, o) = ws(tag::<&str, LocatedSpan<&str>, ()>("test"))(i).unwrap();

        assert_eq!(o.to_string(), "test");
    }

    #[test]
    fn test_quoted() {
        let i = Span::new("\"test\"");

        let (_, o) = quoted(i).unwrap();

        assert_eq!(o.to_string(), "test");
    }

    #[test]
    #[should_panic]
    fn test_quoted_open_left() {
        let i = Span::new("test\"");

        quoted(i).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_quoted_open_right() {
        let i = Span::new("\"test");

        quoted(i).unwrap();
    }

    #[test]
    fn test_parse_namespace() {
        let i = Span::new(indoc! {"
            namespace html = \"http://www.w3.org/1999/xhtml\"
        "});

        let (_, o) = parse_namespace(i).unwrap();

        let r = NamespaceBuilder::default()
            .name("html".into())
            .url(Url::parse("http://www.w3.org/1999/xhtml").unwrap())
            .build()
            .unwrap();

        assert_eq!(o, r);
    }

    #[test]
    fn test_parse_namespaces() {
        let i = Span::new(indoc! {"
            namespace html = \"http://www.w3.org/1999/xhtml\"
            namespace rng = \"http://relaxng.org/ns/structure/1.0\"
        "});

        let (_, o) = parse_namespaces(i).unwrap();

        let r = vec![
            NamespaceBuilder::default()
                .name("html".into())
                .url(Url::parse("http://www.w3.org/1999/xhtml").unwrap())
                .build()
                .unwrap(),
            NamespaceBuilder::default()
                .name("rng".into())
                .url(Url::parse("http://relaxng.org/ns/structure/1.0").unwrap())
                .build()
                .unwrap(),
        ];

        assert_eq!(o, r);
    }
}
