use codegen::Scope;
use convert_case::{Case, Casing};
use log::info;
use quick_xml::{de, se};
use serde::{Deserialize, Serialize};

use self::error::{RelaxNgError, RelaxNgResult};

pub mod error;

#[cfg(test)]
mod tests;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "grammar")]
pub struct Grammar {
    start: Start,
    #[serde(default)]
    define: Vec<Define>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Start {
    #[serde(rename = "$value")]
    pattern: Pattern,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Pattern {
    Empty,
    Ref {
        name: String,
    },
    Group {
        #[serde(rename = "$value")]
        pattern: [Box<Pattern>; 2],
    },
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Define {
    name: String,
    element: Element,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Element {
    name: String,
    #[serde(rename = "$value")]
    pattern: Pattern,
}

pub fn generate(s: &str) -> RelaxNgResult<Grammar> {
    let grammar: Grammar = de::from_str(&s)?;

    Ok(grammar)
}
