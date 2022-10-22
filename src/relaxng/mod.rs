mod parser;

use derive_builder::Builder;
use url::Url;

pub use parser::parse;

#[derive(Debug, PartialEq, Eq, Clone, Builder)]
pub struct Schema {
    pub namespaces: Vec<Namespace>,
}

#[derive(Debug, PartialEq, Eq, Clone, Builder)]
pub struct Namespace {
    pub name: String,
    pub url: Url,
}
