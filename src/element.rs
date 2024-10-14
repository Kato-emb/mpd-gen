pub mod adapt;
pub mod mpd;
pub mod period;
pub mod repr;
pub mod segment;

use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::types::*;

pub trait NeedValidater {
    fn validate(&self) -> Result<(), String>;
}

/// Program Information
#[skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct ProgramInformation {
    #[serde(rename = "@lang")]
    lang: Option<XsLanguage>,
    #[serde(rename = "@moreInformationURL")]
    more_information_url: Option<XsAnyURI>,
    #[serde(rename = "Title")]
    title: Option<String>,
    #[serde(rename = "Source")]
    source: Option<String>,
    #[serde(rename = "Copyright")]
    copyright: Option<String>,
}

/// Patch Location Type
#[skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct PatchLocation {
    #[serde(rename = "$text")]
    base: XsAnyURI,
    #[serde(rename = "@ttl")]
    ttl: Option<f64>,
}

/// Initialization Set
#[skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct InitializationSet {
    #[serde(rename = "@xlink:href", alias = "href")]
    href: Option<String>,
    #[serde(rename = "@xlink:actuate", alias = "actuate")]
    actuate: Option<XLinkActure>,
    #[serde(rename = "@id")]
    id: u32,
    #[serde(rename = "@inAllPeriods")]
    in_all_periods: Option<bool>,
    #[serde(rename = "@contentType")]
    content_type: Option<ContentType>,
    #[serde(rename = "@par")]
    par: Option<Ratio>,
    #[serde(rename = "@maxWidth")]
    max_width: Option<u32>,
    #[serde(rename = "@maxHeight")]
    max_height: Option<u32>,
    #[serde(rename = "@maxFrameRate")]
    max_framerate: Option<FrameRate>,
    #[serde(rename = "@initialization")]
    initialization: Option<XsAnyURI>,
    // common attributes elements
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

/// UInt Vector With ID
#[skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Builder)]
#[builder(setter(into, strip_option), default)]
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

/// Metrics Range
#[skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct MetricsRange {
    #[serde(rename = "@starttime")]
    start_time: Option<XsDuration>,
    #[serde(rename = "@duration")]
    duration: Option<XsDuration>,
}

/// Metrics
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct Metrics {
    #[serde(rename = "@metrics")]
    metrics: String,
    #[serde(rename = "Range", skip_serializing_if = "Vec::is_empty")]
    range: Vec<MetricsRange>,
    #[serde(rename = "Reporting", skip_serializing_if = "Vec::is_empty")]
    reporting: Vec<Descriptor>,
}

/// Leap Second Information
#[skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct LeapSecondInformation {
    #[serde(rename = "@availabilityStartLeapOffset")]
    availability_start_leap_offset: i32,
    #[serde(rename = "@nextAvailabilityStartLeapOffset")]
    next_availability_start_leap_offset: Option<i32>,
    #[serde(rename = "@nextLeapChangeTime")]
    next_leap_change_time: Option<XsDateTime>,
}

/// Table 32
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct Descriptor {
    #[serde(rename = "@schemeIdUri")]
    scheme_id_uri: XsAnyURI,
    #[serde(rename = "@value", skip_serializing_if = "Option::is_none")]
    value: Option<String>,
    #[serde(rename = "@id", skip_serializing_if = "Option::is_none")]
    id: Option<String>,
}

impl From<(String, (Option<String>, Option<String>))> for Descriptor {
    fn from(value: (String, (Option<String>, Option<String>))) -> Self {
        Self {
            scheme_id_uri: value.0.into(),
            value: value.1 .0,
            id: value.1 .1,
        }
    }
}

/// Table 33
///
/// refとref_idはどちらか一方しか存在できない
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct ContentProtection {
    #[serde(flatten)]
    descriptor: Descriptor,
    #[serde(rename = "@ref")]
    r#ref: Option<XsId>,
    #[serde(rename = "@refId")]
    ref_id: Option<XsId>,
    #[serde(rename = "@robustness")]
    robustness: Option<NoWhitespace>,
}

#[skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct Event {
    #[serde(rename = "@presentationTime")]
    presentation_time: Option<u64>,
    #[serde(rename = "@duration")]
    duration: Option<u64>,
    #[serde(rename = "@id")]
    id: Option<u32>,
    #[serde(rename = "@contentEncoding")]
    content_encording: Option<ContentEncoding>,
    /// Deprecated in favor of carrying the message information in the value space of the event
    #[serde(rename = "@messageData")]
    message_data: Option<String>,
}

#[skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct EventStream {
    #[serde(rename = "@xlink:href")]
    href: Option<String>,
    #[serde(rename = "@xlink:actuate")]
    actuate: Option<XLinkActure>,
    #[serde(rename = "@schemeIdUri")]
    scheme_id_uri: XsAnyURI,
    #[serde(rename = "@value")]
    value: Option<String>,
    #[serde(rename = "@timescale")]
    timescale: Option<u32>,
    #[serde(rename = "@presentationTimeOffset")]
    presentation_time_offset: Option<u64>,
    #[serde(rename = "Event", skip_serializing_if = "Vec::is_empty")]
    events: Vec<Event>,
}

#[skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct Switching {
    #[serde(rename = "@interval")]
    interval: u32,
    #[serde(rename = "@type")]
    r#type: Option<SwitchingType>,
}

#[skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct RandomAccess {
    #[serde(rename = "@interval")]
    interval: u32,
    #[serde(rename = "@type")]
    r#type: Option<RandomAccessType>,
    #[serde(rename = "@minBufferTime")]
    min_buffer_time: Option<XsDuration>,
    #[serde(rename = "@bandwidth")]
    bandwidth: Option<u32>,
}

#[skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct Label {
    #[serde(rename = "@id")]
    id: Option<u32>,
    #[serde(rename = "@lang")]
    lang: Option<XsLanguage>,
}

#[skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct ProducerReferenceTime {
    #[serde(rename = "@id")]
    id: u32,
    #[serde(rename = "@inband")]
    inband: Option<bool>,
    #[serde(rename = "@type")]
    r#type: Option<ProducerReferenceTimeType>,
    /// type: applicationの時は必須、それ以外はあってはならない
    #[serde(rename = "@applicationScheme")]
    application_scheme: Option<String>,
    #[serde(rename = "@wallClockTime")]
    wall_clock_time: String,
    #[serde(rename = "@presentationTime")]
    presentation_time: u64,
    #[serde(rename = "UTCTiming")]
    utc_timing: Option<Descriptor>,
}

#[skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct PopularityRate {
    // 1 ~ 100の範囲指定
    #[serde(rename = "@popularityRate")]
    popularity_rate: u32,
    #[serde(rename = "@start")]
    start: Option<u64>,
    #[serde(rename = "@r")]
    repeat_count: Option<i32>,
}

#[skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct ContentPopularityRate {
    #[serde(rename = "@source")]
    source: Source,
    #[serde(rename = "@source_description")]
    source_description: Option<String>,
    #[serde(rename = "PR")]
    popularity_rates: Vec<PopularityRate>,
}

#[skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct Resync {
    #[serde(rename = "@type")]
    r#type: Option<StreamAccessPoint>,
    #[serde(rename = "@dT")]
    diff_time: Option<u32>,
    #[serde(rename = "@dImax")]
    diff_index_max: Option<f32>,
    #[serde(rename = "@dImin")]
    diff_index_min: Option<f32>,
    #[serde(rename = "@marker")]
    marker: Option<bool>,
}

#[skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct BaseURL {
    #[serde(rename = "$text")]
    base: XsAnyURI,
    #[serde(rename = "@serviceLocation")]
    service_location: Option<String>,
    #[serde(rename = "@byteRange")]
    byte_range: Option<String>,
    #[serde(rename = "@availabilityTimeOffset")]
    availability_time_offset: Option<f64>,
    #[serde(rename = "@availabilityTimeComplete")]
    availability_time_complete: Option<bool>,
    #[serde(rename = "@timeShiftBufferDepth")]
    time_shift_buffer_depth: Option<XsDuration>,
    #[serde(rename = "@rangeAccess")]
    range_access: Option<bool>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct ModelPair {
    #[serde(rename = "@bufferTime")]
    buffer_time: XsDuration,
    #[serde(rename = "@bandwidth")]
    bandwidth: u32,
}

#[skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct ExtendedBandwidth {
    #[serde(rename = "@vbr")]
    vbr: Option<bool>,
    #[serde(rename = "ModelPair")]
    model_pair: Vec<ModelPair>,
}

#[skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct ContentComponent {
    #[serde(rename = "@id")]
    id: Option<u32>,
    #[serde(rename = "@lang")]
    lang: Option<XsLanguage>,
    #[serde(rename = "@contentType")]
    content_type: Option<ContentType>,
    #[serde(rename = "@par")]
    par: Option<Ratio>,
    #[serde(rename = "@tag")]
    tag: Option<Tag>,
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
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct Latency {
    #[serde(rename = "@referenceId")]
    reference_id: Option<i32>,
    #[serde(rename = "@target")]
    target_latency: Option<i32>,
    #[serde(rename = "@max")]
    max_latency: Option<i32>,
    #[serde(rename = "@min")]
    min_latency: Option<i32>,
}

#[skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct PlaybackRate {
    #[serde(rename = "@max")]
    max_playback_rate: Option<f32>, // Real
    #[serde(rename = "@min")]
    min_playback_rate: Option<f32>, // Real
}

#[skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct OperatingQuality {
    #[serde(rename = "@mediaType")]
    media_type: Option<QualityMediaType>,
    #[serde(rename = "@min")]
    min_quality_ranking: Option<i32>,
    #[serde(rename = "@max")]
    max_quality_ranking: Option<i32>,
    #[serde(rename = "@target")]
    target_quality_ranking: Option<i32>,
    #[serde(rename = "@type")]
    quality_ranking_type: Option<XsAnyURI>,
    #[serde(rename = "@maxDifference")]
    max_quality_difference: Option<i32>,
}

#[skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct OperatingBandwidth {
    #[serde(rename = "@mediaType")]
    media_type: Option<BandwidthMediaType>,
    #[serde(rename = "@min")]
    min_bandwidth: Option<i32>,
    #[serde(rename = "@max")]
    max_bandwidth: Option<i32>,
    #[serde(rename = "@target")]
    target_bandwidth: Option<i32>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct ServiceDescription {
    #[serde(rename = "@id")]
    id: u32,
    #[serde(rename = "Scope", skip_serializing_if = "Vec::is_empty")]
    scope: Vec<Descriptor>,
    #[serde(rename = "Latency", skip_serializing_if = "Vec::is_empty")]
    latency: Vec<Latency>,
    #[serde(rename = "PlaybackRate", skip_serializing_if = "Vec::is_empty")]
    playback_rate: Vec<PlaybackRate>,
    #[serde(rename = "OperatingQuality", skip_serializing_if = "Vec::is_empty")]
    operating_quality: Vec<OperatingQuality>,
    #[serde(rename = "OperatingBandwidth", skip_serializing_if = "Vec::is_empty")]
    operating_bandwidth: Vec<OperatingBandwidth>,
}

#[skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct Subset {
    #[serde(rename = "@contains")]
    contains: UIntVector,
    #[serde(rename = "@id")]
    id: Option<String>,
}

#[skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct Preselection {
    #[serde(rename = "@id")]
    id: Option<NoWhitespace>,
    #[serde(rename = "@preselectionComponents")]
    preselection_components: Vec<StringVector>,
    #[serde(rename = "@lang")]
    lang: Option<XsLanguage>,
    #[serde(rename = "@order")]
    order: Option<PreselectionOrderType>,
    #[serde(rename = "Accessibility")]
    accessibility: Option<Vec<Descriptor>>,
    #[serde(rename = "Role")]
    role: Option<Vec<Descriptor>>,
    #[serde(rename = "Rating")]
    rating: Option<Vec<Descriptor>>,
    #[serde(rename = "Viewpoint")]
    viewpoint: Option<Vec<Descriptor>>,
    // common attributes elements
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
    // common attributes elements
}

#[skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct Url {
    #[serde(rename = "@sourceURL")]
    pub source_url: Option<XsAnyURI>,
    #[serde(rename = "@range")]
    pub range: Option<SingleByteRange>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Builder)]
#[builder(setter(into, strip_option), default)]
#[serde(rename = "FCS")]
pub struct Fcs {
    #[serde(rename = "@t")]
    pub start_time: u64,
    #[serde(rename = "@d", skip_serializing_if = "Option::is_none")]
    pub duration: Option<u64>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Builder)]
#[builder(setter(into, strip_option), default)]
#[serde(rename = "FailoverContent")]
pub struct FailoverContent {
    #[serde(rename = "@valid")]
    pub valid: Option<bool>,
    #[serde(rename = "FCS", skip_serializing_if = "Vec::is_empty")]
    pub fcs_list: Vec<Fcs>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Builder)]
#[builder(setter(into, strip_option), default)]
#[serde(rename = "SegmentURL")]
pub struct SegmentUrl {
    #[serde(rename = "@media")]
    media: Option<XsAnyURI>,
    #[serde(rename = "@mediaRange")]
    media_range: Option<SingleByteRange>,
    #[serde(rename = "@index")]
    index: Option<XsAnyURI>,
    #[serde(rename = "@indexRange")]
    index_range: Option<SingleByteRange>,
}
