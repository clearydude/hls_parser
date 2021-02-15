use thiserror::Error;

pub(crate) type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Error, Debug)]
pub(crate) enum Error {
    #[error("API Error: {0}")]
    APIError(String),
    #[error("Parse Error: {0}")]
    ParseError(String),
}