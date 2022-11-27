pub mod errors;
pub mod parser;

#[cfg(test)]
mod tests;

use std::io::Read;

use self::{errors::RelaxNgResult, parser::parse};

pub fn generate<R>(mut r: R) -> RelaxNgResult<()>
where
    R: Read,
{
    let mut buffer = String::new();
    r.read_to_string(&mut buffer)?;

    parse(&buffer)
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Decl {
    Namespace(String, String),
    DefaultNamespace(Option<String>, String),
    Datatypes(String, String),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum AssignMethod {
    Assign,
    And,
    Or,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum GrammarContent {
    Start(AssignMethod, Pattern),
    Define,
    Div,
    Include,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Pattern {
    Element,
    Attribute,
    List,
    And,
    Or,
    OneOrZero,
    Many0,
    Many1,
    Lst,
    Mixed,
    Identifier(String),
    Parent(String),
    Empty,
    Text,
    Datatype,
    DataTypeExcl,
    NotAllowed,
    External,
    Grammar(Vec<GrammarContent>),
}
