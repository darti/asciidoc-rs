use derive_builder::Builder;

use quick_xml::de;
use serde::{Deserialize, Serialize};

use self::error::RelaxNgResult;

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
        #[serde(default)]
        #[serde(rename = "@name")]
        name: Option<String>,

        #[serde(alias = "anyName")]
        // #[serde(alias = "name")]
        #[serde(alias = "choice")]
        #[serde(default)]
        #[serde(rename = "$value")]
        name_class: Option<NameClass>,

        #[serde(alias = "ref")]
        #[serde(rename = "$value", default)]
        pattern: Vec<Pattern>,
    },

    Attribute {
        #[serde(default)]
        #[serde(rename = "@name")]
        name: Option<String>,

        #[serde(alias = "anyName")]
        // #[serde(alias = "name")]
        #[serde(alias = "choice")]
        #[serde(default)]
        name_class: Option<NameClass>,

        #[serde(default)]
        pattern: Option<Box<Pattern>>,
    },

    Text,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum NameClass {
    Name(String),

    AnyName {
        // #[serde(rename = "$unflatten=except", default)]
        except: Option<Box<NameClass>>,
    },
    // NsName {
    //     #[serde(default)]
    //     except: Option<ExceptNameClass>,
    // },
    Choice {
        #[serde(rename = "$value", default)]
        names: Vec<NameClass>,
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
        name: Some(name.into()),
        name_class: None,
        pattern,
    }
}

pub fn attribute(name: &str, pattern: Option<Pattern>) -> Pattern {
    Pattern::Attribute {
        name: Some(name.into()),
        name_class: None,
        pattern: pattern.map(Box::new),
    }
}

pub fn attribute_with_name_class(name_class: NameClass, pattern: Option<Pattern>) -> Pattern {
    Pattern::Attribute {
        name: None,
        name_class: Some(name_class),
        pattern: pattern.map(Box::new),
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

pub fn generate(s: &str) -> RelaxNgResult<Grammar> {
    let grammar: Grammar = de::from_str(&s)?;

    Ok(grammar)
}
