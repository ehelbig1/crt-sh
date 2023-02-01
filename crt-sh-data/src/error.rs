use thiserror::Error;

#[derive(Debug, Error, Clone)]
pub enum Error {
    #[error("Request Error")]
    RequestError,

    #[error("Parse Error")]
    ParseError,
}
