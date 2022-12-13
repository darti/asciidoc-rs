mod common;
mod declarations;
mod grammar;
mod identifiers;
mod names;
mod namespaces;
mod pattern;

use log::info;

use nom_locate::LocatedSpan;

use nom::branch::alt;

use crate::relaxng::parser::declarations::decls;

use super::errors::RelaxNgResult;

pub type Span<'a> = LocatedSpan<&'a str>;

pub fn parse(s: &str) -> RelaxNgResult<()> {
    let input = Span::new(s);

    let r = decls(input);

    info!("{:?}", r.ok());

    Ok(())
}
