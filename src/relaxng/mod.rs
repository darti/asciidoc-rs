pub mod error;
mod parser;

use std::collections::HashMap;

use derive_builder::Builder;
use url::Url;

use self::error::{RelaxNgError, RelaxNgResult};

#[derive(Debug, PartialEq, Eq, Clone, Builder, Default)]
pub struct Schema {
    pub namespaces: Namespaces,
}

#[derive(Debug, PartialEq, Eq, Clone, Builder, Default)]
pub struct Namespaces {
    pub namespaces: HashMap<String, Namespace>,
    pub default_namespace: Option<String>,
}

impl Namespaces {
    pub fn insert(&mut self, namespace: &Namespace, is_default: bool) {
        self.namespaces
            .insert(namespace.name.to_string(), namespace.to_owned());

        if is_default {
            self.default_namespace = Some(namespace.name.to_string());
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Builder)]
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
