use nom::{
    branch::alt,
    bytes::complete::{escaped, tag, take_until},
    character::complete::{char, multispace0, one_of},
    combinator::map,
    sequence::{delimited, terminated},
    IResult,
};

use crate::relaxng::AssignMethod;

use super::Span;

pub(crate) fn assign_method(input: Span) -> IResult<Span, AssignMethod> {
    alt((
        map(tag("="), |_| AssignMethod::Assign),
        map(tag("|="), |_| AssignMethod::Or),
        map(tag("&="), |_| AssignMethod::And),
    ))(input)
}

pub(crate) fn quoted(input: Span) -> IResult<Span, Span> {
    delimited(
        char('"'),
        escaped(take_until("\""), '\\', one_of(r#""n\"#)),
        terminated(char('"'), multispace0),
    )(input)
}

#[cfg(test)]
mod tests {}
