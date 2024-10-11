pub mod adapt;
pub mod mpd;
pub mod period;
pub mod repr;
pub mod segment;

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::types::*;

#[skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ProgramInformation {
    #[serde(rename = "@lang")]
    lang: Option<XsLanguage>,
    #[serde(rename = "@moreInformationURL")]
    more_information_url: Option<AnyUri>,
    #[serde(rename = "Title")]
    title: Option<String>,
    #[serde(rename = "Source")]
    source: Option<String>,
    #[serde(rename = "Copyright")]
    copyright: Option<String>,
}

#[skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct PatchLocation {
    #[serde(rename = "$text")]
    base: AnyUri,
    #[serde(rename = "@ttl")]
    ttl: Option<f64>,
}

#[skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct InitializationSet {
    #[serde(rename = "@xlink:href")]
    href: Option<String>,
    #[serde(rename = "@xlink:actuate")]
    actuate: Option<XLinkActure>,
    #[serde(rename = "@id")]
    id: u32,
    #[serde(rename = "@inAllPeriods")]
    in_all_periods: Option<bool>,
    #[serde(rename = "@contentType")]
    content_type: Option<ContentType>,
    #[serde(rename = "@par")]
    par: Option<AspectRatio>,
    #[serde(rename = "@maxWidth")]
    max_width: Option<u32>,
    #[serde(rename = "@maxHeight")]
    max_height: Option<u32>,
    #[serde(rename = "@maxFrameRate")]
    max_framerate: Option<FrameRate>,
    #[serde(rename = "@initialization")]
    initialization: Option<AnyUri>,
    // common attributes elements
    #[serde(rename = "@profiles")]
    profiles: Option<ListOfProfiles>,
    #[serde(rename = "@width")]
    width: Option<u32>,
    #[serde(rename = "@height")]
    height: Option<u32>,
    #[serde(rename = "@sar")]
    sar: Option<AspectRatio>,
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
    // common attributes elements
    #[serde(rename = "Accessibility")]
    accessibility: Option<Vec<Descriptor>>,
    #[serde(rename = "Role")]
    role: Option<Vec<Descriptor>>,
    #[serde(rename = "Rating")]
    rating: Option<Vec<Descriptor>>,
    #[serde(rename = "Viewpoint")]
    viewpoint: Option<Vec<Descriptor>>,
}

#[skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct UIntVWithID {
    #[serde(rename = "$value")]
    base: UIntVector,
    #[serde(rename = "@id")]
    id: u32,
    #[serde(rename = "@profiles", skip_serializing_if = "Vec::is_empty")]
    profiles: Vec<ListOfProfiles>,
    #[serde(rename = "@contentType")]
    content_type: Option<ContentType>,
}

#[skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MetricsRange {
    #[serde(rename = "@starttime")]
    start_time: Option<XsDuration>,
    #[serde(rename = "@duration")]
    duration: Option<XsDuration>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Metrics {
    #[serde(rename = "@metrics")]
    metrics: String,
    #[serde(rename = "Range", skip_serializing_if = "Vec::is_empty")]
    range: Vec<MetricsRange>,
    #[serde(rename = "Reporting", skip_serializing_if = "Vec::is_empty")]
    reporting: Vec<Descriptor>,
}

#[skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct LeapSecondInformation {
    #[serde(rename = "@availabilityStartLeapOffset")]
    availability_start_leap_offset: i32,
    #[serde(rename = "@nextAvailabilityStartLeapOffset")]
    next_availability_start_leap_offset: Option<i32>,
    #[serde(rename = "@nextLeapChangeTime")]
    next_leap_change_time: Option<XsDateTime>,
}
