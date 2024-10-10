mod common;
mod element;
mod error;
mod scheme;
mod types;

pub use element::segment::{Segment, SegmentBuilder, SegmentTimeline, SegmentTimelineBuilder};
use error::MpdError;

pub type Result<T> = std::result::Result<T, MpdError>;
