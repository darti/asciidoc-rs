#[cfg(test)]
mod tests;

mod helpers;

use nom::character::complete::{multispace0, multispace1};

use nom::multi::{many0, many_m_n};

use nom::{
    bytes::complete::tag, bytes::complete::tag_no_case, character::complete::alphanumeric1,
    combinator::map_res, sequence::separated_pair, IResult,
};

use nom_locate::LocatedSpan;

use self::helpers::{quoted, skip_comment_lines, ws};

use super::error::{RelaxNgError, RelaxNgResult};
use super::{Namespace, Schema, SchemaBuilder};

pub type Span<'a> = LocatedSpan<&'a str>;

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
    many0(skip_comment_lines(parse_namespace))(input)
}

pub fn parse(s: &str) -> RelaxNgResult<Schema> {
    let input = Span::new(s);

    let mut schema = SchemaBuilder::default();

    let (input, ns) = parse_namespaces(input).map_err(|e| e.map_input(|i| i.to_string()))?;

    for (d, n) in ns {
        if d {
            schema.default_namespace(n.name.to_owned());
        }

        schema.namespace((n.name.to_owned(), n));
    }

    schema.build()
}
