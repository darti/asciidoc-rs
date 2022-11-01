use quick_xml::DeError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RelaxNgError {
    #[error("invalid url")]
    InvalidUrl(#[from] url::ParseError),

    #[error("builder failure, missing field: {0}")]
    BuilderMissingField(&'static str),

    #[error("element with no name")]
    ElementWithNoName,

    #[error("unsupported root node, grammar only")]
    Unsupported,

    #[error("missing <start /> in <grammar /> ")]
    MissingStart,

    #[error(transparent)]
    DeserializerError(#[from] DeError),
}

pub type RelaxNgResult<T> = Result<T, RelaxNgError>;

impl From<derive_builder::UninitializedFieldError> for RelaxNgError {
    fn from(err: derive_builder::UninitializedFieldError) -> Self {
        Self::BuilderMissingField(err.field_name())
    }
}
