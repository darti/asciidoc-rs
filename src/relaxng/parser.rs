use nom::bytes::complete::take_until;
use nom::character::complete::{multispace0, multispace1};
use nom::error::Error as NomError;
use nom::sequence::tuple;
use nom::{
    bytes::complete::tag,
    bytes::complete::tag_no_case,
    character::complete::{alphanumeric0, alphanumeric1, char},
    combinator::map_res,
    sequence::{delimited, separated_pair},
    IResult,
};

use nom_locate::LocatedSpan;
use url::Url;

use super::{Namespace, NamespaceBuilder, Schema, SchemaBuilder};

type Span<'a> = LocatedSpan<&'a str>;

fn parse_namespace(input: Span) -> IResult<Span, Namespace> {
    let (input, _) = tag_no_case("namespace")(input)?;
    let (input, _) = multispace1(input)?;

    map_res(
        separated_pair(
            alphanumeric1,
            tuple((multispace0, tag("="), multispace0)),
            delimited(char('"'), take_until("\""), char('"')),
        ),
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

        // assert_eq!();
    }
}
