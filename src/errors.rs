use thiserror::Error;

pub(crate) type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Error, Debug, PartialEq)]
pub enum Error {
    #[error("API Error: {0}")]
    HTTP(String),
    #[error("Parse Error: {0}")]
    Parse(String),
    #[error("Invalid HLS: {0}")]
    HLSFormat(String),
}
