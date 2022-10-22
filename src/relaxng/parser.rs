use nom::bytes::complete::{escaped, take_until};
use nom::character::complete::{multispace0, multispace1, one_of};
use nom::error::Error as NomError;
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

fn quoted<'a, E>() -> impl FnMut(Span<'a>) -> IResult<Span<'a>, Span<'a>, E>
where
    E: nom::error::ParseError<Span<'a>>,
{
    delimited(
        char('"'),
        escaped(take_until("\""), '\\', one_of(r#""n\"#)),
        char('"'),
    )
}

fn parse_namespace(input: Span) -> IResult<Span, Namespace> {
    let (input, _) = tag_no_case("namespace")(input)?;
    let (input, _) = multispace1(input)?;

    map_res(
        separated_pair(alphanumeric1, ws(tag("=")), quoted()),
        |(k, v): (Span, Span)| {
            Ok::<Namespace, NomError<Span>>(Namespace {
                name: k.to_string(),
                url: Url::parse(&v).unwrap(),
            })
        },
    )(input)
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
}
