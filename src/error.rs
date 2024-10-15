use thiserror::Error;

#[derive(Debug, Error)]
pub enum MpdError {
    #[error("{0}")]
    InvalidData(&'static str),
    #[error("Unmatched the pattern defined in the XML schema.")]
    UnmatchedPattern,
    #[error("{0}")]
    UninitializedFieldError(#[from] derive_builder::UninitializedFieldError),
    #[error("{0}")]
    ValidationError(&'static str),
    #[error("{0}")]
    IoError(#[from] std::io::Error),
    #[error("{0}")]
    ParseIntError(#[from] std::num::ParseIntError),
    #[error("{0}")]
    ParseFloatError(#[from] std::num::ParseFloatError),
    #[error("{0}")]
    ChronoParseError(#[from] chrono::format::ParseError),
    #[error("{0}")]
    QuickXmlSerdeError(#[from] quick_xml::DeError),
}
