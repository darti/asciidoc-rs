pub mod error;
mod parser;

pub use parser::parse;

use std::collections::HashMap;

use derive_builder::Builder;
use url::Url;

use self::error::{RelaxNgError, RelaxNgResult};

#[derive(Debug, PartialEq, Clone, Builder, Default)]
#[builder(build_fn(error = "RelaxNgError"), default)]
pub struct Schema {
    #[builder(setter(each(name = "namespace")))]
    namespaces: HashMap<String, Namespace>,

    #[builder(setter(strip_option, into))]
    default_namespace: Option<String>,
}

#[derive(Debug, PartialEq, Clone)]

pub struct Namespace {
    pub name: String,
    pub url: Url,
}

impl Namespace {
    pub fn new(name: &str, url: &str) -> RelaxNgResult<Self> {
        Ok(Self {
            name: name.to_string(),
            url: Url::parse(url)?,
        })
    }
}
