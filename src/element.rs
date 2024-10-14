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
#[builder(
    setter(into, strip_option),
    default,
    build_fn(validate = "Self::validate")
)]
pub struct UIntVWithID {
    #[serde(rename = "$text")]
    base: UIntVector,
    #[serde(rename = "@id")]
    id: u32,
    #[serde(rename = "@profiles")]
    profiles: Option<Vec<ListOfProfiles>>,
    #[serde(rename = "@contentType")]
    content_type: Option<ContentType>,
}

impl NeedValidater for UIntVWithIDBuilder {
    fn validate(&self) -> Result<(), String> {
        if self.id.is_none() {
            Err("Must be set a unique unsigned integer identifier".to_string())
        } else {
            Ok(())
        }
    }
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
#[skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Builder)]
#[builder(
    setter(into, strip_option),
    default,
    build_fn(validate = "Self::validate")
)]
pub struct Metrics {
    #[serde(rename = "@metrics")]
    metrics: String,
    #[serde(rename = "Range")]
    range: Option<Vec<MetricsRange>>,
    #[serde(rename = "Reporting")]
    reporting: Vec<Descriptor>,
}

impl NeedValidater for MetricsBuilder {
    fn validate(&self) -> Result<(), String> {
        if self.metrics.is_none() {
            Err("Metrics must be set @metrics attribute".to_string())
        } else if !self.reporting.as_ref().is_some_and(|rep| !rep.is_empty()) {
            Err("Metrics must be set Reporting element longer than 0".to_string())
        } else {
            Ok(())
        }
    }
}

/// Leap Second Information
#[skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Builder)]
#[builder(
    setter(into, strip_option),
    default,
    build_fn(validate = "Self::validate")
)]
pub struct LeapSecondInformation {
    #[serde(rename = "@availabilityStartLeapOffset")]
    availability_start_leap_offset: i32,
    #[serde(rename = "@nextAvailabilityStartLeapOffset")]
    next_availability_start_leap_offset: Option<i32>,
    #[serde(rename = "@nextLeapChangeTime")]
    next_leap_change_time: Option<XsDateTime>,
}

impl NeedValidater for LeapSecondInformationBuilder {
    fn validate(&self) -> Result<(), String> {
        if self.availability_start_leap_offset.is_none() {
            Err("LeapSecondInformation must be set @availabilityStartLeapOffset".to_string())
        } else {
            Ok(())
        }
    }
}

/// Descriptor
///
/// Table 32
#[skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Builder)]
#[builder(
    setter(into, strip_option),
    default,
    build_fn(validate = "Self::validate")
)]
pub struct Descriptor {
    #[serde(rename = "@schemeIdUri")]
    scheme_id_uri: XsAnyURI,
    #[serde(rename = "@value", skip_serializing_if = "Option::is_none")]
    value: Option<String>,
    #[serde(rename = "@id", skip_serializing_if = "Option::is_none")]
    id: Option<String>,
}

impl NeedValidater for DescriptorBuilder {
    fn validate(&self) -> Result<(), String> {
        if self.scheme_id_uri.is_none() {
            Err("Descriptor must be set @schemeIdUri".to_string())
        } else {
            Ok(())
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
#[builder(
    setter(into, strip_option),
    default,
    build_fn(validate = "Self::validate")
)]
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
    #[serde(rename = "Event")]
    events: Option<Vec<Event>>,
}

impl NeedValidater for EventStreamBuilder {
    fn validate(&self) -> Result<(), String> {
        if self.scheme_id_uri.is_none() {
            Err("EventStream must be set @schemeIdUri".to_string())
        } else {
            Ok(())
        }
    }
}

#[skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Builder)]
#[builder(
    setter(into, strip_option),
    default,
    build_fn(validate = "Self::validate")
)]
pub struct Switching {
    #[serde(rename = "@interval")]
    interval: u32,
    #[serde(rename = "@type")]
    r#type: Option<SwitchingType>,
}

impl NeedValidater for SwitchingBuilder {
    fn validate(&self) -> Result<(), String> {
        if self.interval.is_none() {
            Err("Switching must be set @interval".to_string())
        } else {
            Ok(())
        }
    }
}

#[skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Builder)]
#[builder(
    setter(into, strip_option),
    default,
    build_fn(validate = "Self::validate")
)]
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

impl NeedValidater for RandomAccessBuilder {
    fn validate(&self) -> Result<(), String> {
        if self.interval.is_none() {
            Err("RandomAccess must be set @interval".to_string())
        } else {
            Ok(())
        }
    }
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

pub type GroupLavel = Label;

#[skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Builder)]
#[builder(
    setter(into, strip_option),
    default,
    build_fn(validate = "Self::validate")
)]
pub struct ProducerReferenceTime {
    #[serde(rename = "@id")]
    id: u32,
    #[serde(rename = "@inband")]
    inband: Option<bool>,
    #[serde(rename = "@type")]
    r#type: Option<ProducerReferenceTimeType>,
    #[serde(rename = "@applicationScheme")]
    application_scheme: Option<String>,
    #[serde(rename = "@wallClockTime")]
    wall_clock_time: String,
    #[serde(rename = "@presentationTime")]
    presentation_time: u64,
    #[serde(rename = "UTCTiming")]
    utc_timing: Option<Descriptor>,
}

impl NeedValidater for ProducerReferenceTimeBuilder {
    fn validate(&self) -> Result<(), String> {
        if self.id.is_none() || self.wall_clock_time.is_none() || self.presentation_time.is_none() {
            Err(
                "ProducerReferenceTime must be set @id, @wallClockTime and @presentationTime"
                    .to_string(),
            )
        } else if self
            .r#type
            .as_ref()
            .is_some_and(|typ| typ == &Some(ProducerReferenceTimeType::Application))
            && self.application_scheme.is_none()
        {
            Err(
                "If the @type is set other than application, this attribute shall not be present"
                    .to_string(),
            )
        } else {
            Ok(())
        }
    }
}

#[skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Builder)]
#[builder(
    setter(into, strip_option),
    default,
    build_fn(validate = "Self::validate")
)]
pub struct PopularityRate {
    // 1 ~ 100の範囲指定
    #[serde(rename = "@popularityRate")]
    popularity_rate: u32,
    #[serde(rename = "@start")]
    start: Option<u64>,
    #[serde(rename = "@r")]
    repeat_count: Option<i32>,
}

impl NeedValidater for PopularityRateBuilder {
    fn validate(&self) -> Result<(), String> {
        match self.popularity_rate.as_ref() {
            Some(rate) => {
                if !(1..=100).contains(rate) {
                    Err("The value shall be in the range of 1 to 100.".to_string())
                } else {
                    Ok(())
                }
            }
            None => Err("PopularityRate must be set @popularityRate".to_string()),
        }
    }
}

#[skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Builder)]
#[builder(
    setter(into, strip_option),
    default,
    build_fn(validate = "Self::validate")
)]
pub struct ContentPopularityRate {
    #[serde(rename = "@source")]
    source: Source,
    #[serde(rename = "@source_description")]
    source_description: Option<String>,
    #[serde(rename = "PR")]
    popularity_rates: Vec<PopularityRate>,
}

impl NeedValidater for ContentPopularityRateBuilder {
    fn validate(&self) -> Result<(), String> {
        if self.source.is_none() {
            Err("ContentPopularityRate must be set @source".to_string())
        } else if !self
            .popularity_rates
            .as_ref()
            .is_some_and(|rates| !rates.is_empty())
        {
            Err("ContentPopularityRate must be set PR longer than 0".to_string())
        } else {
            Ok(())
        }
    }
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
#[builder(
    setter(into, strip_option),
    default,
    build_fn(validate = "Self::validate")
)]
pub struct ModelPair {
    #[serde(rename = "@bufferTime")]
    buffer_time: XsDuration,
    #[serde(rename = "@bandwidth")]
    bandwidth: u32,
}

impl NeedValidater for ModelPairBuilder {
    fn validate(&self) -> Result<(), String> {
        if self.buffer_time.is_none() || self.bandwidth.is_none() {
            Err("ModelPair must be set @bufferTime and @bandwidth".to_string())
        } else {
            Ok(())
        }
    }
}

#[skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct ExtendedBandwidth {
    #[serde(rename = "@vbr")]
    vbr: Option<bool>,
    #[serde(rename = "ModelPair")]
    model_pair: Option<Vec<ModelPair>>,
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

#[skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Builder)]
#[builder(
    setter(into, strip_option),
    default,
    build_fn(validate = "Self::validate")
)]
pub struct ServiceDescription {
    #[serde(rename = "@id")]
    id: u32,
    #[serde(rename = "Scope")]
    scope: Option<Vec<Descriptor>>,
    #[serde(rename = "Latency")]
    latency: Option<Vec<Latency>>,
    #[serde(rename = "PlaybackRate")]
    playback_rate: Option<Vec<PlaybackRate>>,
    #[serde(rename = "OperatingQuality")]
    operating_quality: Option<Vec<OperatingQuality>>,
    #[serde(rename = "OperatingBandwidth")]
    operating_bandwidth: Option<Vec<OperatingBandwidth>>,
}

impl NeedValidater for ServiceDescriptionBuilder {
    fn validate(&self) -> Result<(), String> {
        if self.id.is_none() {
            Err("ServiceDescription must be set @id".to_string())
        } else {
            Ok(())
        }
    }
}

#[skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Builder)]
#[builder(
    setter(into, strip_option),
    default,
    build_fn(validate = "Self::validate")
)]
pub struct Subset {
    #[serde(rename = "@contains")]
    contains: UIntVector,
    #[serde(rename = "@id")]
    id: Option<String>,
}

impl NeedValidater for SubsetBuilder {
    fn validate(&self) -> Result<(), String> {
        if self.contains.is_none() {
            Err("Subset must be set @contains".to_string())
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
pub struct Preselection {
    #[serde(rename = "@id")]
    id: Option<NoWhitespace>,
    #[serde(rename = "@preselectionComponents")]
    preselection_components: StringVector,
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

impl NeedValidater for PreselectionBuilder {
    fn validate(&self) -> Result<(), String> {
        if self.preselection_components.is_none() {
            Err("Preselection must be set @preselectionComponents".to_string())
        } else {
            Ok(())
        }
    }
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

#[skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Builder)]
#[builder(
    setter(into, strip_option),
    default,
    build_fn(validate = "Self::validate")
)]
pub struct Fcs {
    #[serde(rename = "@t")]
    pub start_time: u64,
    #[serde(rename = "@d")]
    pub duration: Option<u64>,
}

impl NeedValidater for FcsBuilder {
    fn validate(&self) -> Result<(), String> {
        if self.start_time.is_none() {
            Err("FCS must be set @t".to_string())
        } else {
            Ok(())
        }
    }
}

#[skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Builder)]
#[builder(
    setter(into, strip_option),
    default,
    build_fn(validate = "Self::validate")
)]
pub struct FailoverContent {
    #[serde(rename = "@valid")]
    pub valid: Option<bool>,
    #[serde(rename = "FCS")]
    pub fcs_list: Vec<Fcs>,
}

impl NeedValidater for FailoverContentBuilder {
    fn validate(&self) -> Result<(), String> {
        if !self.fcs_list.as_ref().is_some_and(|list| !list.is_empty()) {
            Err("FailoverContent must be set FCS longer than 0".to_string())
        } else {
            Ok(())
        }
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_element_u_int_v_with_id_valid() {
        assert!(UIntVWithIDBuilder::default().id(1u32).build().is_ok());
        assert!(UIntVWithIDBuilder::default().build().is_err());
    }

    #[test]
    fn test_element_metrics_valid() {
        let reporting = DescriptorBuilder::default()
            .scheme_id_uri("https://example.com")
            .build()
            .unwrap();
        assert!(MetricsBuilder::default()
            .metrics("metric_key1,metric_key2")
            .reporting(vec![reporting.clone()])
            .build()
            .is_ok());
        assert!(MetricsBuilder::default().build().is_err());
        assert!(MetricsBuilder::default()
            .metrics("metric_key1,metric_key2")
            .build()
            .is_err());
        assert!(MetricsBuilder::default()
            .reporting(vec![reporting])
            .build()
            .is_err());
    }

    #[test]
    fn test_element_leap_second_information_valid() {
        assert!(LeapSecondInformationBuilder::default()
            .availability_start_leap_offset(1i32)
            .build()
            .is_ok());
        assert!(LeapSecondInformationBuilder::default().build().is_err());
    }

    #[test]
    fn test_element_descriptor_valid() {
        assert!(DescriptorBuilder::default()
            .scheme_id_uri("https://example.com")
            .build()
            .is_ok());
        assert!(DescriptorBuilder::default().build().is_err());
    }

    #[test]
    fn test_element_event_stream_valid() {
        assert!(EventStreamBuilder::default()
            .scheme_id_uri("https://example.com")
            .build()
            .is_ok());
        assert!(EventStreamBuilder::default().build().is_err());
    }

    #[test]
    fn test_element_switching_valid() {
        assert!(SwitchingBuilder::default().interval(1u32).build().is_ok());
        assert!(SwitchingBuilder::default().build().is_err());
    }

    #[test]
    fn test_element_random_access_valid() {
        assert!(RandomAccessBuilder::default()
            .interval(1u32)
            .build()
            .is_ok());
        assert!(RandomAccessBuilder::default().build().is_err());
    }

    #[test]
    fn test_element_producer_reference_time_valid() {
        assert!(ProducerReferenceTimeBuilder::default()
            .id(1u32)
            .wall_clock_time("12345")
            .presentation_time(1u64)
            .build()
            .is_ok());
        assert!(ProducerReferenceTimeBuilder::default().build().is_err());
        assert!(ProducerReferenceTimeBuilder::default()
            .id(1u32)
            .wall_clock_time("12345")
            .presentation_time(1u64)
            .r#type(ProducerReferenceTimeType::Application)
            .application_scheme("https://example.com")
            .build()
            .is_ok());
        assert!(ProducerReferenceTimeBuilder::default()
            .id(1u32)
            .wall_clock_time("12345")
            .presentation_time(1u64)
            .r#type(ProducerReferenceTimeType::Application)
            .build()
            .is_err());
    }

    #[test]
    fn test_element_popularity_rate_valid() {
        assert!(PopularityRateBuilder::default()
            .popularity_rate(1u32)
            .build()
            .is_ok());
        assert!(PopularityRateBuilder::default().build().is_err());
        assert!(PopularityRateBuilder::default()
            .popularity_rate(0u32)
            .build()
            .is_err());
    }

    #[test]
    fn test_element_content_popularity_rate_valid() {
        let popularity_rate = PopularityRateBuilder::default()
            .popularity_rate(1u32)
            .build()
            .unwrap();

        assert!(ContentPopularityRateBuilder::default()
            .source(Source::Content)
            .popularity_rates(vec![popularity_rate.clone()])
            .build()
            .is_ok());
        assert!(ContentPopularityRateBuilder::default()
            .source(Source::Content)
            .build()
            .is_err());
        assert!(ContentPopularityRateBuilder::default()
            .popularity_rates(vec![popularity_rate.clone()])
            .build()
            .is_err());
    }

    #[test]
    fn test_element_model_pair_vaild() {
        assert!(ModelPairBuilder::default()
            .buffer_time(std::time::Duration::from_secs(5))
            .bandwidth(2_000_000u32)
            .build()
            .is_ok());
        assert!(ModelPairBuilder::default().build().is_err());
        assert!(ModelPairBuilder::default()
            .buffer_time(std::time::Duration::from_secs(5))
            .build()
            .is_err());
        assert!(ModelPairBuilder::default()
            .bandwidth(2_000_000u32)
            .build()
            .is_err());
    }

    #[test]
    fn test_element_service_description_vaild() {
        assert!(ServiceDescriptionBuilder::default()
            .id(1u32)
            .build()
            .is_ok());
        assert!(ServiceDescriptionBuilder::default().build().is_err());
    }

    #[test]
    fn test_element_subset_valid() {
        let contains = UIntVector::from([1u32, 2, 3].as_slice());
        assert!(SubsetBuilder::default().contains(contains).build().is_ok());
        assert!(SubsetBuilder::default().build().is_err());
    }

    #[test]
    fn test_element_preselection_valid() {
        let preselection_components = StringVector::from(["id_1", "id_2"].as_slice());
        assert!(PreselectionBuilder::default()
            .preselection_components(preselection_components)
            .build()
            .is_ok());
        assert!(PreselectionBuilder::default().build().is_err());
    }

    #[test]
    fn test_element_fcs_valid() {
        assert!(FcsBuilder::default().start_time(1u64).build().is_ok());
        assert!(FcsBuilder::default().build().is_err());
    }

    #[test]
    fn test_element_failover_content_valid() {
        let fcs = FcsBuilder::default().start_time(1u64).build().unwrap();
        assert!(FailoverContentBuilder::default()
            .fcs_list(vec![fcs])
            .build()
            .is_ok());
        assert!(FailoverContentBuilder::default().build().is_err());
    }
}
