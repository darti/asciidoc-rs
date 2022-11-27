use log::info;
use nom::{
    bytes::complete::{escaped, tag, take_until},
    character::complete::{self, alphanumeric1, char, multispace0, multispace1, one_of},
    combinator::{map_opt, opt},
    multi::{many0, separated_list0},
    sequence::{delimited, preceded, separated_pair, terminated},
    IResult,
};
use nom_locate::LocatedSpan;

use nom::branch::alt;

use super::{errors::RelaxNgResult, Decl};

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
