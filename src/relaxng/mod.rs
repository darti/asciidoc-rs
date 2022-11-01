use codegen::Scope;
use convert_case::{Case, Casing};
use derive_builder::Builder;
use log::info;
use quick_xml::{de, se};
use serde::{Deserialize, Serialize};

use self::error::{RelaxNgError, RelaxNgResult};

pub mod error;

#[cfg(test)]
mod tests;

#[derive(Debug, PartialEq, Serialize, Deserialize, Builder)]
#[serde(rename = "grammar")]
pub struct Grammar {
    #[builder(default)]
    start: Start,

    #[serde(default, rename = "define")]
    #[builder(default, setter(each(name = "define")))]
    defines: Vec<Define>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Default)]
pub struct Start {
    #[serde(rename = "$value")]
    pattern: Pattern,
}

impl Start {
    pub fn new(pattern: Pattern) -> Self {
        Self { pattern }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum Pattern {
    Empty,
    Ref {
        name: String,
    },
    ExternalRef {
        href: String,
    },
    Group {
        #[serde(rename = "$value", default)]
        pattern: [Box<Pattern>; 2],
    },
    ZeroOrMore {
        #[serde(rename = "$value", default)]
        pattern: Vec<Pattern>,
    },
    Choice {
        #[serde(rename = "$value", default)]
        pattern: Vec<Pattern>,
    },

    Element {
        name: String,
        #[serde(rename = "$value", default)]
        pattern: Vec<Pattern>,
    },

    Attribute {
        #[serde(default)]
        name: Option<String>,
        #[serde(default)]
        name_class: Option<NameClass>,

        #[serde(rename = "$value", default)]
        pattern: Box<Pattern>,
    },
}

impl Default for Pattern {
    fn default() -> Self {
        Pattern::Empty
    }
}

pub fn empty() -> Pattern {
    Pattern::Empty
}

pub fn reference(name: &str) -> Pattern {
    Pattern::Ref { name: name.into() }
}

pub fn external_reference(href: &str) -> Pattern {
    Pattern::ExternalRef { href: href.into() }
}

pub fn group(pattern: [Box<Pattern>; 2]) -> Pattern {
    Pattern::Group { pattern }
}

pub fn zero_or_more(pattern: Vec<Pattern>) -> Pattern {
    Pattern::ZeroOrMore { pattern }
}

pub fn choice(pattern: Vec<Pattern>) -> Pattern {
    Pattern::Choice { pattern }
}

pub fn element(name: &str, pattern: Vec<Pattern>) -> Pattern {
    Pattern::Element {
        name: name.into(),
        pattern,
    }
}

pub fn attribute(name: &str, pattern: Pattern) -> Pattern {
    Pattern::Attribute {
        name: Some(name.into()),
        name_class: None,
        pattern: Box::new(pattern),
    }
}

pub fn attribute_with_name_class(name_class: NameClass, pattern: Pattern) -> Pattern {
    Pattern::Attribute {
        name: None,
        name_class: Some(name_class),
        pattern: Box::new(pattern),
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Define {
    name: String,
    #[serde(rename = "$value", default)]
    pattern: Vec<Pattern>,
}

impl Define {
    pub fn new(name: &str, pattern: Vec<Pattern>) -> Self {
        Self {
            name: name.into(),
            pattern,
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum NameClass {
    Name(String),
    AnyName {
        except: Option<Box<NameClass>>,
    },
    Choice {
        #[serde(rename = "$value", default)]
        names: Vec<NameClass>,
    },
    Except(Vec<NameClass>),
}

pub fn generate(s: &str) -> RelaxNgResult<Grammar> {
    let grammar: Grammar = de::from_str(&s)?;

    Ok(grammar)
}
