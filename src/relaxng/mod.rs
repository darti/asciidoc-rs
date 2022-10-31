use serde::{Deserialize, Serialize};
use serde_with::serde_as;

pub mod error;

#[cfg(test)]
mod tests;

#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Default)]
pub struct Grammar {
    #[serde(rename = "$value")]
    #[serde(default)]
    pub grammar_content: Vec<GrammarContent>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(untagged)]
pub enum GrammarContent {
    Start(Start),
    // Define(Define),
    // Include(Include),
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Start {
    pattern: Pattern,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Define {}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Default)]
pub struct Div {}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Default)]
pub struct Include {}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Default)]
pub struct Choice {}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub enum Pattern {
    Element,
    Attribute,
    Group,
    Interleave,
    Choice(Choice),
    Optional,
    ZeroOrMore,
    OneOrMore,
    List,
    Mixed,
    Ref,
    ParentRef,
    Empty,
    Text,
    Value,
    Data,
    NotAllowed,
    ExternalRef,
    Grammar(Grammar),
}
