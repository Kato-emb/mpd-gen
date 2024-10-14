use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::element::*;
use crate::types::*;

use super::segment::{SegmentBase, SegmentList, SegmentTemplate};

#[skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Builder)]
#[builder(
    setter(into, strip_option),
    default,
    build_fn(validate = "Self::validate")
)]
pub struct Representation {
    #[serde(rename = "@id")]
    id: NoWhitespace,
    #[serde(rename = "@bandwidth")]
    bandwidth: u32,
    #[serde(rename = "@qualityRanking")]
    quality_ranking: Option<u32>,
    #[serde(rename = "@dependencyId")]
    dependency_id: Option<StringVector>,
    #[serde(rename = "@associationId")]
    association_id: Option<StringVector>,
    #[serde(rename = "@associationType")]
    association_type: Option<ListOfFourCC>,
    #[serde(rename = "@mediaStreamStructureId")]
    media_stream_structure_id: Option<StringVector>,
    // Common attributes and elements
    #[serde(rename = "@profiles")]
    profiles: Option<ListOfProfiles>,
    #[serde(rename = "@width")]
    width: Option<u32>,
    #[serde(rename = "@height")]
    height: Option<u32>,
    #[serde(rename = "@sar")]
    sar: Option<Ratio>,
    #[serde(rename = "@frameRate")]
    framerate: Option<FrameRate>,
    #[serde(rename = "@audioSamplingRate")]
    audio_sampling_rate: Option<AudioSamplingRate>,
    #[serde(rename = "@mimeType")]
    mime_type: Option<String>,
    #[serde(rename = "@segmentProfiles")]
    segment_profiles: Option<ListOfFourCC>,
    #[serde(rename = "@codecs")]
    codecs: Option<Codecs>,
    #[serde(rename = "@containerProfiles")]
    container_profiles: Option<ListOfFourCC>,
    #[serde(rename = "@maximumSAPPeriod")]
    maximum_sap_period: Option<f64>,
    #[serde(rename = "@startWithSAP")]
    start_with_sap: Option<StreamAccessPoint>,
    #[serde(rename = "@maxPlayoutRate")]
    max_playout_rate: Option<f64>,
    #[serde(rename = "@codingDependency")]
    coding_dependency: Option<bool>,
    #[serde(rename = "@scanType")]
    scan_type: Option<VideoScan>,
    #[serde(rename = "@selectionPriority")]
    selection_priority: Option<u32>,
    #[serde(rename = "@tag")]
    tag: Option<Tag>,
    #[serde(rename = "FramePacking")]
    frame_packing: Option<Vec<Descriptor>>,
    #[serde(rename = "AudioChannelConfiguration")]
    audio_channel_configuration: Option<Vec<Descriptor>>,
    #[serde(rename = "ContentProtection")]
    content_protection: Option<Vec<ContentProtection>>,
    #[serde(rename = "OutputProtection")]
    output_protection: Option<Vec<Descriptor>>,
    #[serde(rename = "EssentialProperty")]
    essential_property: Option<Vec<Descriptor>>,
    #[serde(rename = "SupplementalProperty")]
    supplemental_property: Option<Vec<Descriptor>>,
    #[serde(rename = "InbandEventStream")]
    inband_event_stream: Option<Vec<EventStream>>,
    #[serde(rename = "Switching")]
    switching: Option<Vec<Switching>>,
    #[serde(rename = "RandomAccess")]
    random_access: Option<Vec<RandomAccess>>,
    #[serde(rename = "GroupLabel")]
    group_lavel: Option<Vec<Label>>,
    #[serde(rename = "Label")]
    lavel: Option<Vec<Label>>,
    #[serde(rename = "ProducerReferenceTime")]
    producer_reference_time: Option<Vec<ProducerReferenceTime>>,
    #[serde(rename = "ContentPopularityRate")]
    content_popularity_rate: Option<Vec<ContentPopularityRate>>,
    #[serde(rename = "Resync")]
    resync: Option<Vec<Resync>>,
    // Common attributes and elements
    #[serde(rename = "BaseURL")]
    base_url: Option<Vec<BaseURL>>,
    #[serde(rename = "ExtendedBandwidth")]
    extended_bandwidth: Option<Vec<ExtendedBandwidth>>,
    #[serde(rename = "SubRepresentation")]
    sub_representation: Option<Vec<SubRepresentation>>,
    #[serde(rename = "SegmentBase")]
    segment_base: Option<SegmentBase>,
    #[serde(rename = "SegmentList")]
    segment_list: Option<SegmentList>,
    #[serde(rename = "SegmentTemplate")]
    segment_template: Option<SegmentTemplate>,
}

impl NeedValidater for RepresentationBuilder {
    fn validate(&self) -> Result<(), String> {
        if self.id.is_none() || self.bandwidth.is_none() {
            Err("Representation must be set @id and @bandwidth".to_string())
        } else {
            Ok(())
        }
    }
}

#[skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Builder)]
#[builder(
    setter(into, strip_option),
    default,
    build_fn(validate = "Self::validate")
)]
pub struct SubRepresentation {
    #[serde(rename = "@level")]
    level: Option<u32>,
    #[serde(rename = "@dependencyLevel")]
    dependency_level: Option<UIntVector>,
    // lavelが存在する場合に存在する
    #[serde(rename = "@bandwidth")]
    bandwidth: Option<u32>,
    #[serde(rename = "@contentComponent")]
    content_component: Option<StringVector>,
    #[serde(rename = "@profiles")]
    profiles: Option<ListOfProfiles>,
    #[serde(rename = "@width")]
    width: Option<u32>,
    #[serde(rename = "@height")]
    height: Option<u32>,
    #[serde(rename = "@sar")]
    sar: Option<Ratio>,
    #[serde(rename = "@frameRate")]
    framerate: Option<FrameRate>,
    #[serde(rename = "@audioSamplingRate")]
    audio_sampling_rate: Option<AudioSamplingRate>,
    #[serde(rename = "@mimeType")]
    mime_type: Option<String>,
    #[serde(rename = "@segmentProfiles")]
    segment_profiles: Option<ListOfFourCC>,
    #[serde(rename = "@codecs")]
    codecs: Option<Codecs>,
    #[serde(rename = "@containerProfiles")]
    container_profiles: Option<ListOfFourCC>,
    #[serde(rename = "@maximumSAPPeriod")]
    maximum_sap_period: Option<f64>,
    #[serde(rename = "@startWithSAP")]
    start_with_sap: Option<StreamAccessPoint>,
    #[serde(rename = "@maxPlayoutRate")]
    max_playout_rate: Option<f64>,
    #[serde(rename = "@codingDependency")]
    coding_dependency: Option<bool>,
    #[serde(rename = "@scanType")]
    scan_type: Option<VideoScan>,
    #[serde(rename = "@selectionPriority")]
    selection_priority: Option<u32>,
    #[serde(rename = "@tag")]
    tag: Option<Tag>,
    #[serde(rename = "FramePacking")]
    frame_packing: Option<Vec<Descriptor>>,
    #[serde(rename = "AudioChannelConfiguration")]
    audio_channel_configuration: Option<Vec<Descriptor>>,
    #[serde(rename = "ContentProtection")]
    content_protection: Option<Vec<ContentProtection>>,
    #[serde(rename = "OutputProtection")]
    output_protection: Option<Vec<Descriptor>>,
    #[serde(rename = "EssentialProperty")]
    essential_property: Option<Vec<Descriptor>>,
    #[serde(rename = "SupplementalProperty")]
    supplemental_property: Option<Vec<Descriptor>>,
    #[serde(rename = "InbandEventStream")]
    inband_event_stream: Option<Vec<EventStream>>,
    #[serde(rename = "Switching")]
    switching: Option<Vec<Switching>>,
    #[serde(rename = "RandomAccess")]
    random_access: Option<Vec<RandomAccess>>,
    #[serde(rename = "GroupLabel")]
    group_lavel: Option<Vec<GroupLavel>>,
    #[serde(rename = "Label")]
    lavel: Option<Vec<Label>>,
    #[serde(rename = "ProducerReferenceTime")]
    producer_reference_time: Option<Vec<ProducerReferenceTime>>,
    #[serde(rename = "ContentPopularityRate")]
    content_popularity_rate: Option<Vec<ContentPopularityRate>>,
    #[serde(rename = "Resync")]
    resync: Option<Vec<Resync>>,
}

impl NeedValidater for SubRepresentationBuilder {
    fn validate(&self) -> Result<(), String> {
        if self.level.is_some() && self.bandwidth.is_none() {
            Err("This attribute shall be present if the @level attribute is present.".to_string())
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
    fn test_element_representation_vaild() {
        assert!(RepresentationBuilder::default()
            .id(NoWhitespace::from_str("720p").unwrap())
            .bandwidth(2_000_000u32)
            .build()
            .is_ok());
        assert!(RepresentationBuilder::default().build().is_err());
    }

    #[test]
    fn test_element_sub_representation_vaild() {
        assert!(SubRepresentationBuilder::default().build().is_ok());
        assert!(SubRepresentationBuilder::default()
            .level(1u32)
            .bandwidth(2_000_000u32)
            .build()
            .is_ok());
        assert!(SubRepresentationBuilder::default()
            .level(1u32)
            .build()
            .is_err());
    }
}
