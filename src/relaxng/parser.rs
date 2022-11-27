use log::info;
use nom::{
    bytes::complete::{escaped, is_not, tag, take_until},
    character::complete::{self, alphanumeric1, char, multispace0, multispace1, one_of},
    combinator::{map, map_opt, not, opt, recognize, verify},
    error::ParseError,
    multi::{many0, separated_list0},
    sequence::{delimited, preceded, separated_pair, terminated},
    IResult,
};
use nom_locate::LocatedSpan;

use nom::branch::alt;

use super::{errors::RelaxNgResult, AssignMethod, Decl, GrammarContent, Pattern};

pub type Span<'a> = LocatedSpan<&'a str>;

pub fn parse(s: &str) -> RelaxNgResult<()> {
    let input = Span::new(s);

    let r = decls(input);

    info!("{:?}", r.ok());

    Ok(())
}

pub(crate) fn quoted(input: Span) -> IResult<Span, Span> {
    delimited(
        char('"'),
        escaped(take_until("\""), '\\', one_of(r#""n\"#)),
        terminated(complete::char('"'), complete::multispace0),
    )(input)
}

pub(crate) fn namespace(input: Span) -> IResult<Span, Decl> {
    let (input, _) = terminated(tag("namespace"), multispace0)(input)?;

    let (input, id) = terminated(alphanumeric1, multispace0)(input)?;
    let (input, _) = terminated(char('='), multispace0)(input)?;
    let (input, ns) = terminated(quoted, multispace0)(input)?;

    Ok((input, Decl::Namespace(id.to_string(), ns.to_string())))
}

pub(crate) fn default_namespace(input: Span) -> IResult<Span, Decl> {
    let (input, _) = terminated(tag("default"), multispace0)(input)?;
    let (input, _) = terminated(tag("namespace"), multispace0)(input)?;

    let (input, id) = opt(terminated(alphanumeric1, multispace0))(input)?;
    let (input, _) = terminated(char('='), multispace0)(input)?;
    let (input, ns) = terminated(quoted, multispace0)(input)?;

    Ok((
        input,
        Decl::DefaultNamespace(id.map(|i| i.to_string()), ns.to_string()),
    ))
}

pub(crate) fn datatypes(input: Span) -> IResult<Span, Decl> {
    let (input, _) = terminated(tag("datatypes"), multispace0)(input)?;

    let (input, id) = terminated(alphanumeric1, multispace0)(input)?;
    let (input, _) = terminated(char('='), multispace0)(input)?;
    let (input, ns) = terminated(quoted, multispace0)(input)?;

    Ok((input, Decl::Datatypes(id.to_string(), ns.to_string())))
}

pub(crate) fn decls(input: Span) -> IResult<Span, Vec<Decl>> {
    many0(alt((namespace, default_namespace, datatypes)))(input)
}

pub(crate) fn grammar_contents(input: Span) -> IResult<Span, Vec<GrammarContent>> {
    many0(alt((start, define, div, include)))(input)
}

pub(crate) fn assign_method(input: Span) -> IResult<Span, AssignMethod> {
    alt((
        map(tag("="), |_| AssignMethod::Assign),
        map(tag("|="), |_| AssignMethod::Or),
        map(tag("&="), |_| AssignMethod::And),
    ))(input)
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

pub(crate) fn pattern(input: Span) -> IResult<Span, Pattern> {
    alt((map(identifier, |i| Pattern::Identifier(i.to_string())),))(input)
}

pub(crate) fn identifier(input: Span) -> IResult<Span, Span> {
    alphanumeric1(input)
}

pub(crate) fn keyword(input: Span) -> IResult<Span, Span> {
    recognize(alt((
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
    )))(input)
}
