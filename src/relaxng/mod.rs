use codegen::Scope;
use convert_case::{Case, Casing};
use log::info;
use quick_xml::{de, se};
use serde::{de::IntoDeserializer, Deserialize, Serialize};

use self::error::{RelaxNgError, RelaxNgResult};

pub mod error;

#[cfg(test)]
mod tests;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "grammar")]
pub struct Grammar {
    start: Start,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Start {
    #[serde(rename = "$value")]
    pattern: Pattern,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Pattern {
    Ref { name: String },
}

pub fn generate(s: &str) -> RelaxNgResult<Grammar> {
    let grammar: Grammar = de::from_str(&s)?;

    Ok(grammar)
}
