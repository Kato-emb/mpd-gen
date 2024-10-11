use thiserror::Error;

#[derive(Debug, Error)]
pub enum MpdError {
    #[error("{0}")]
    InvalidData(&'static str),
    #[error("Unmatched the pattern defined in the XML schema.")]
    UnmatchedPattern,
    #[error("{0}")]
    ParseError(#[from] std::num::ParseIntError),
    #[error("{0}")]
    ChronoParseError(#[from] chrono::format::ParseError),
}
