use nom::bytes::complete::tag;
use nom::character::complete::multispace0;
use nom::multi::many0;
use nom::sequence::terminated;
use nom::IResult;

use crate::relaxng::{parser::*, *};

use super::common::assign_method;
use super::pattern::pattern;

pub(crate) fn grammar_contents(input: Span) -> IResult<Span, Vec<GrammarContent>> {
    many0(alt((start, define, div, include)))(input)
}

pub(crate) fn start(input: Span) -> IResult<Span, GrammarContent> {
    let (input, _) = terminated(tag("start"), multispace0)(input)?;

    let (input, op) = terminated(assign_method, multispace0)(input)?;
    let (input, pattern) = terminated(pattern, multispace0)(input)?;

    Ok((input, GrammarContent::Start(op, pattern)))
}

pub(crate) fn define(input: Span) -> IResult<Span, GrammarContent> {
    todo!()
}

pub(crate) fn div(input: Span) -> IResult<Span, GrammarContent> {
    todo!()
}

pub(crate) fn include(input: Span) -> IResult<Span, GrammarContent> {
    todo!()
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use crate::relaxng::{
        parser::{grammar::start, Span},
        AssignMethod, GrammarContent, Pattern,
    };

    #[test]
    fn test_start() {
        let i = Span::new(indoc! {r#"
        start = pattern
    "#});

        let (_, o) = start(i).unwrap();

        assert_eq!(
            o,
            GrammarContent::Start(AssignMethod::Assign, Pattern::Identifier("pattern".into()))
        );
    }
}
