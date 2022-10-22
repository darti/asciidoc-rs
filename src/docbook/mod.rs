use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct Article {
    info: Option<Info>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct Info {}
