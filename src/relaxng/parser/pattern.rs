use nom::{branch::alt, combinator::map, IResult};

use crate::relaxng::Pattern;

use super::{identifiers::identifier, Span};

pub(crate) fn pattern(input: Span) -> IResult<Span, Pattern> {
    // let elt = seq(terminated(tag("element"), multispace0),);
    let id = map(identifier, |i| Pattern::Identifier(i.to_string()));

    alt((id,))(input)
}

#[cfg(test)]
mod tests {}
