mod element;
mod entity;
mod error;
mod scheme;
mod types;

#[macro_use]
mod macros;

pub use element::segment::{Segment, SegmentBuilder, SegmentTimeline, SegmentTimelineBuilder};
use error::MpdError;

pub type Result<T> = std::result::Result<T, MpdError>;
