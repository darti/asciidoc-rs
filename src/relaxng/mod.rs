use derive_builder::Builder;

use serde::{Deserialize, Serialize};

pub mod error;

#[cfg(test)]
mod tests;

#[derive(Debug, Serialize, Deserialize, PartialEq, Builder, Clone, Default)]
#[serde(rename = "grammar")]
pub struct Grammar {
    #[serde(rename = "$value")]
    #[builder(default, setter(strip_option, each(name = "content")))]
    pub grammar_content: Option<Vec<GrammarContent>>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub enum GrammarContent {
    Start(Start),
    Define(Define),
    Include(Include),
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Builder, Clone, Default)]
pub struct Start {}

#[derive(Debug, Serialize, Deserialize, PartialEq, Builder, Clone, Default)]
pub struct Define {}

#[derive(Debug, Serialize, Deserialize, PartialEq, Builder, Clone, Default)]
pub struct Div {}

#[derive(Debug, Serialize, Deserialize, PartialEq, Builder, Clone, Default)]
pub struct Include {}

#[derive(Debug, Serialize, Deserialize, PartialEq, Builder, Clone, Default)]
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
