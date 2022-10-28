use thiserror::Error;

#[derive(Debug, Error)]
pub enum RelaxNgError {
    #[error("invalid url")]
    InvalidUrl(#[from] url::ParseError),

    #[error(transparent)]
    ParseError(#[from] nom::Err<nom::error::Error<String>>),

    #[error("builder failure, missing field: {0}")]
    BuilderMissingField(&'static str),
}

pub type RelaxNgResult<T> = Result<T, RelaxNgError>;

impl From<derive_builder::UninitializedFieldError> for RelaxNgError {
    fn from(err: derive_builder::UninitializedFieldError) -> Self {
        Self::BuilderMissingField(err.field_name())
    }
}
