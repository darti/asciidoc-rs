#[cfg(test)]
mod tests;

use nom::branch::alt;
use nom::bytes::complete::{escaped, take, take_until};
use nom::character::complete::{multispace0, multispace1, one_of};
use nom::combinator::{opt, value};
use nom::multi::{many0, many_m_n};
use nom::sequence::{pair, preceded, terminated, tuple};
use nom::{
    bytes::complete::tag,
    bytes::complete::tag_no_case,
    character::complete::{alphanumeric1, char},
    combinator::map_res,
    sequence::{delimited, separated_pair},
    IResult,
};

use nom_locate::LocatedSpan;

use super::error::RelaxNgError;
use super::Namespace;

pub type Span<'a> = LocatedSpan<&'a str>;

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

fn comment(input: Span) -> IResult<Span, ()> {
    value((), tuple((ws(tag("#")), take_until("\n"), take(1usize))))(input)
}

fn skip_comments_eol<'a, F, O>(inner: F) -> impl FnMut(Span<'a>) -> IResult<Span<'a>, O>
where
    F: Fn(Span<'a>) -> IResult<Span<'a>, O>,
{
    terminated(ws(inner), opt(comment))
}

fn skip_comments_line<'a, F, O>(inner: F) -> impl FnMut(Span<'a>) -> IResult<Span<'a>, O>
where
    F: Fn(Span<'a>) -> IResult<Span<'a>, O>,
{
    preceded(many0(comment), inner)
}

fn parse_namespace(input: Span) -> IResult<Span, (bool, Namespace)> {
    let (input, is_default) = map_res(many_m_n(0, 1, tag("default")), |d| {
        Ok::<bool, RelaxNgError>(d.len() > 0)
    })(input)?;
    let (input, _) = multispace0(input)?;
    let (input, _) = tag_no_case("namespace")(input)?;
    let (input, _) = multispace1(input)?;

    map_res(
        separated_pair(alphanumeric1, ws(tag("=")), quoted),
        move |(k, v): (Span, Span)| Namespace::new(&k, &v).map(|n| (is_default, n)),
    )(input)
}

fn parse_namespaces(input: Span) -> IResult<Span, Vec<(bool, Namespace)>> {
    many0(skip_comments_eol(parse_namespace))(input)
}

pub fn parse(s: &str) {
    let span = Span::new(s);

    // SchemaBuilder::default().build();
}
