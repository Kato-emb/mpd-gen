use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::types::{FailoverContent, SingleRFC7233RangeType, Url, XsDuration, XsInteger};

#[skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct SegmentBaseInformation {
    #[serde(rename = "@timescale")]
    timescale: Option<u32>,
    #[serde(rename = "@presentationTimeOffset")]
    presentation_time_offset: Option<u64>,
    #[serde(rename = "@eptDelta")]
    ept_delta: Option<XsInteger>,
    #[serde(rename = "@pdDelta")]
    pd_delta: Option<XsInteger>,
    #[serde(rename = "@presentationDuration")]
    presentation_duration: Option<u64>,
    #[serde(rename = "@timeShiftBufferDepth")]
    time_shift_buffer_depth: Option<XsDuration>,
    #[serde(rename = "@indexRange")]
    index_range: Option<SingleRFC7233RangeType>,
    #[serde(rename = "@indexRangeExact")]
    index_range_exact: Option<bool>,
    #[serde(rename = "@availabilityTimeOffset")]
    availability_time_offset: Option<f64>,
    #[serde(rename = "@availabilityTimeComplete")]
    availability_time_complete: Option<bool>,
    #[serde(rename = "Initialization")]
    initialization: Option<Url>,
    #[serde(rename = "RepresentationIndex")]
    representation_index: Option<Url>,
    #[serde(rename = "FailoverContent")]
    failover_content: Option<FailoverContent>,
}

#[skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct MultipleSegmentBaseInformation {
    #[serde(rename = "@duration")]
    duration: Option<u32>,
    #[serde(rename = "@startNumber")]
    start_number: Option<u32>,
    #[serde(rename = "@endNumber")]
    end_number: Option<u32>,
    #[serde(flatten)]
    segment_base_information: SegmentBaseInformation,
    #[serde(rename = "SegmentTimeline")]
    segment_timeline: Option<SegmentTimeline>,
    #[serde(rename = "BitstreamSwitching")]
    bitstream_switching: Option<Url>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct SegmentBase {
    #[serde(flatten)]
    segment_base_information: SegmentBaseInformation,
}

/// Attribute name is `SegmentTimeline`
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct SegmentTimeline {
    #[builder(setter(custom))]
    #[serde(rename = "S", skip_serializing_if = "Vec::is_empty")]
    segments: Vec<Segment>,
}

impl SegmentTimelineBuilder {
    pub fn segment(&mut self, segment: Segment) -> &mut Self {
        self.segments.get_or_insert_with(Vec::new).push(segment);
        self
    }

    pub fn segments<S>(&mut self, segments: S) -> &mut Self
    where
        S: IntoIterator<Item = Segment>,
    {
        self.segments.get_or_insert_with(Vec::new).extend(segments);
        self
    }
}

/// Attribute name is `S`
#[skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Builder)]
#[builder(setter(into, strip_option), default)]
#[serde(rename = "S")]
pub struct Segment {
    #[serde(rename = "@t")]
    start_time: Option<u64>,
    #[serde(rename = "@n")]
    number: Option<u64>,
    #[serde(rename = "@d")]
    duration: u64,
    #[serde(rename = "@k")]
    segment_count: Option<u64>,
    #[serde(rename = "@r")]
    repeat_count: Option<XsInteger>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_element_segment_timeline() {
        let segment1 = SegmentBuilder::default()
            .duration(5u64)
            .start_time(0u64)
            .repeat_count(10)
            .build()
            .unwrap();
        let segment2 = SegmentBuilder::default()
            .duration(5u64)
            .repeat_count(10)
            .build()
            .unwrap();
        let segment3 = SegmentBuilder::default()
            .duration(5u64)
            .repeat_count(15)
            .build()
            .unwrap();

        let segment_timeline = SegmentTimelineBuilder::default()
            .segments([segment1, segment2])
            .segment(segment3)
            .build()
            .unwrap();

        let mut xml = String::new();
        let mut ser = quick_xml::se::Serializer::new(&mut xml);
        ser.indent(' ', 2);
        segment_timeline.serialize(ser).unwrap();

        let se = r#"<SegmentTimeline>
  <S t="0" d="5" r="10"/>
  <S d="5" r="10"/>
  <S d="5" r="15"/>
</SegmentTimeline>"#;

        assert!(&xml == se);
    }

    #[test]
    fn test_element_segment_base() {
        let base = SegmentBaseInformation::default();
        println!("{}", serde_plain::to_string(&base).unwrap());

        let mut xml = String::new();
        let mut ser = quick_xml::se::Serializer::new(&mut xml);
        ser.indent(' ', 2);
        base.serialize(ser).unwrap();

        println!("{xml}");
    }
}

// pub trait SegmentBaseInformation {
//     fn timescale(&mut self, timescale: u32) -> &mut Self;
//     fn presentation_time_offset(&mut self, presentation_time_offset: u64) -> &mut Self;
//     fn ept_delta(&mut self, ept_delta: i64) -> &mut Self;
//     fn pd_delta(&mut self, pd_delta: i64) -> &mut Self;
//     fn presentation_duration(&mut self, presentation_duration: u64) -> &mut Self;
//     fn time_shift_buffer_depth<D: Into<XsDuration>>(
//         &mut self,
//         time_shift_buffer_depth: D,
//     ) -> &mut Self;
//     fn index_range(&mut self, start: Option<u64>, end: Option<u64>) -> &mut Self;
//     fn index_range_exact(&mut self, index_range_exact: bool) -> &mut Self;
//     fn availability_time_offset(&mut self, availability_time_offset: f64) -> &mut Self;
//     fn availability_time_complete(&mut self, availability_time_complete: bool) -> &mut Self;
//     fn initialization(
//         &mut self,
//         source_uri: String,
//         start: Option<u64>,
//         end: Option<u64>,
//     ) -> &mut Self;
// }
