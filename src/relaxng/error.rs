use thiserror::Error;

#[derive(Debug, Error)]
pub enum RelaxNgError {
    #[error("invalid url")]
    InvalidUrl(#[from] url::ParseError),
}

pub type RelaxNgResult<T> = Result<T, RelaxNgError>;
