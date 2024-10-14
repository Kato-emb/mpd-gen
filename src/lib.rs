mod definition;
mod element;
mod entity;
mod error;
mod types;

#[macro_use]
mod macros;

pub use element::adapt::*;
pub use element::mpd::*;
pub use element::period::*;
pub use element::repr::*;
pub use element::segment::*;
pub use element::*;

pub use definition::*;
pub use types::*;

pub use error::MpdError;

pub type Result<T> = std::result::Result<T, MpdError>;
