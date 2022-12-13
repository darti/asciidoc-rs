use nom::{
    branch::alt,
    character::complete::{char, multispace0},
    combinator::map,
    sequence::delimited,
    IResult,
};

use crate::relaxng::NameClass;

use super::{identifiers::identifier_or_keyword, Span};

pub(crate) fn name_class(input: Span) -> IResult<Span, NameClass> {
    let name = map(identifier_or_keyword, |n| NameClass::Name(n.to_string()));

    alt((
        name,
        delimited(
            char('('),
            delimited(multispace0, name_class, multispace0),
            char(')'),
        ),
    ))(input)
}

#[cfg(test)]
mod tests {}
