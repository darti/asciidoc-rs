pub mod error;

use relaxng_syntax::{
    compact::{schema, Span},
    types::Schema,
};

use self::error::RelaxNgResult;

pub fn generate(s: &str) -> RelaxNgResult<Schema> {
    schema(Span::new(s)).map_err(|_| error::RelaxNgError::ParseError)
}
