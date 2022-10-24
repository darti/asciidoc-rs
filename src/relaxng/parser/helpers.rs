use nom::bytes::complete::{escaped, take, take_until};

use nom::character::complete::{multispace0, one_of};
use nom::combinator::{opt, value};
use nom::multi::many0;
use nom::sequence::{preceded, terminated, tuple};
use nom::{bytes::complete::tag, character::complete::char, sequence::delimited, IResult};

use super::Span;

pub(crate) fn ws<'a, F, O, E>(inner: F) -> impl FnMut(Span<'a>) -> IResult<Span<'a>, O, E>
where
    F: Fn(Span<'a>) -> IResult<Span<'a>, O, E>,
    E: nom::error::ParseError<Span<'a>>,
{
    delimited(multispace0, inner, multispace0)
}

pub(crate) fn quoted(input: Span) -> IResult<Span, Span> {
    delimited(
        char('"'),
        escaped(take_until("\""), '\\', one_of(r#""n\"#)),
        char('"'),
    )(input)
}

pub(crate) fn comment(input: Span) -> IResult<Span, ()> {
    value((), tuple((ws(tag("#")), take_until("\n"), take(1usize))))(input)
}

pub(crate) fn skip_comments_eol<'a, F, O>(inner: F) -> impl FnMut(Span<'a>) -> IResult<Span<'a>, O>
where
    F: Fn(Span<'a>) -> IResult<Span<'a>, O>,
{
    terminated(ws(inner), opt(comment))
}

pub(crate) fn skip_comment_lines<'a, F, O>(inner: F) -> impl FnMut(Span<'a>) -> IResult<Span<'a>, O>
where
    F: Fn(Span<'a>) -> IResult<Span<'a>, O>,
{
    preceded(many0(comment), ws(inner))
}
