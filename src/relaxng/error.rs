use thiserror::Error;

use super::parser::Span;

#[derive(Debug, Error)]
pub enum RelaxNgError {
    #[error("invalid url")]
    InvalidUrl(#[from] url::ParseError),

    #[error(transparent)]
    ParseError(#[from] nom::error::Error<String>),
}

pub type RelaxNgResult<T> = Result<T, RelaxNgError>;
