use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::element::*;
use crate::types::*;

#[skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct SegmentBase {
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
    index_range: Option<SingleByteRange>,
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
pub struct SegmentList {
    #[serde(rename = "@xlink:href")]
    href: Option<String>,
    #[serde(rename = "@xlink:actuate")]
    actuate: Option<String>,
    #[serde(rename = "@duration")]
    duration: Option<u32>,
    #[serde(rename = "@startNumber")]
    start_number: Option<u32>,
    #[serde(rename = "@endNumber")]
    end_number: Option<u32>,
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
    index_range: Option<SingleByteRange>,
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
    #[serde(rename = "SegmentTimeline")]
    segment_timeline: Option<SegmentTimeline>,
    #[serde(rename = "BitstreamSwitching")]
    bitstream_switching: Option<Url>,
    #[serde(rename = "SegmentURL", skip_serializing_if = "Vec::is_empty", default)]
    segment_url: Vec<SegmentUrl>,
}

#[skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct SegmentTemplate {
    #[serde(rename = "@duration")]
    duration: Option<u32>,
    #[serde(rename = "@startNumber")]
    start_number: Option<u32>,
    #[serde(rename = "@endNumber")]
    end_number: Option<u32>,
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
    index_range: Option<SingleByteRange>,
    #[serde(rename = "@indexRangeExact")]
    index_range_exact: Option<bool>,
    #[serde(rename = "@availabilityTimeOffset")]
    availability_time_offset: Option<f64>,
    #[serde(rename = "@availabilityTimeComplete")]
    availability_time_complete: Option<bool>,
    #[serde(rename = "Initialization")]
    initialization_element: Option<Url>,
    #[serde(rename = "RepresentationIndex")]
    representation_index: Option<Url>,
    #[serde(rename = "FailoverContent")]
    failover_content: Option<FailoverContent>,
    #[serde(rename = "SegmentTimeline")]
    segment_timeline: Option<SegmentTimeline>,
    #[serde(rename = "BitstreamSwitching")]
    bitstream_switching_element: Option<Url>,
    #[serde(rename = "@media")]
    media: Option<String>,
    #[serde(rename = "@index")]
    index: Option<String>,
    #[serde(rename = "@initialization")]
    initialization_attribute: Option<String>,
    #[serde(rename = "@bitstreamSwitching")]
    bitstream_switching_attribute: Option<String>,
}

/// Attribute name is `SegmentTimeline`
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct SegmentTimeline {
    #[builder(setter(custom))]
    #[serde(rename = "S", skip_serializing_if = "Vec::is_empty", default)]
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
#[builder(
    setter(into, strip_option),
    default,
    build_fn(validate = "Self::validate", error = "MpdError")
)]
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

impl CustomValidate for SegmentBuilder {
    fn validate(&self) -> Result<()> {
        if self.duration == None || self.duration == Some(0) {
            Err(MpdError::ValidationError(
                "Segment duration must be set longer than 0",
            ))
        } else {
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn test_element_segment_valid() {
        assert!(SegmentBuilder::default().duration(1u64).build().is_ok());
        assert!(SegmentBuilder::default().build().is_err());
    }

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
        let segment_base = SegmentBaseBuilder::default()
            .timescale(3000u32)
            .availability_time_offset(10.1)
            .time_shift_buffer_depth(XsDuration::from_str("PT3H11M53S").unwrap())
            .initialization(
                UrlBuilder::default()
                    .source_url("http://example.com/video.mp4")
                    .range(SingleByteRange::from_str("0-200").unwrap())
                    .build()
                    .unwrap(),
            )
            .build()
            .unwrap();

        let mut xml = String::new();
        let mut ser = quick_xml::se::Serializer::new(&mut xml);
        ser.indent(' ', 2);
        segment_base.serialize(ser).unwrap();

        let der = quick_xml::de::from_str::<SegmentBase>(&xml).unwrap();
        println!("{xml}");

        assert_eq!(segment_base, der);
    }

    #[test]
    fn test_element_segment_list() {
        let segment = SegmentBuilder::default()
            .duration(5u64)
            .start_time(0u64)
            .repeat_count(10)
            .build()
            .unwrap();

        let segment_timeline = SegmentTimelineBuilder::default()
            .segments([segment])
            .build()
            .unwrap();

        let segment_list = SegmentListBuilder::default()
            .duration(5000u32)
            .start_number(1u32)
            .availability_time_offset(10000)
            // .initialization((
            //     Some("http://example.com/video.mp4".to_string()),
            //     (Some(100), Some(200)),
            // ))
            .segment_timeline(segment_timeline)
            .build()
            .unwrap();

        let mut xml = String::new();
        let mut ser = quick_xml::se::Serializer::new(&mut xml);
        ser.indent(' ', 2);
        segment_list.serialize(ser).unwrap();

        let der = quick_xml::de::from_str::<SegmentList>(&xml).unwrap();

        assert_eq!(segment_list, der);
    }

    #[test]
    fn test_element_segment_template() {
        let segment = SegmentBuilder::default()
            .duration(5u64)
            .start_time(0u64)
            .repeat_count(10)
            .build()
            .unwrap();

        let segment_timeline = SegmentTimelineBuilder::default()
            .segments([segment])
            .build()
            .unwrap();

        let segment_template = SegmentTemplateBuilder::default()
            .start_number(1u32)
            .presentation_time_offset(1000u32)
            .segment_timeline(segment_timeline)
            .build()
            .unwrap();

        let mut xml = String::new();
        let mut ser = quick_xml::se::Serializer::new(&mut xml);
        ser.indent(' ', 2);
        segment_template.serialize(ser).unwrap();

        let der = quick_xml::de::from_str::<SegmentTemplate>(&xml).unwrap();

        assert_eq!(segment_template, der);
    }
}
