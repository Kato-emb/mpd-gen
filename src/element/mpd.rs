use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::types::*;

use super::{
    period::Period, InitializationSet, LeapSecondInformation, Metrics, PatchLocation,
    ProgramInformation, UIntVWithID,
};

#[skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct MPD {
    #[serde(rename = "@id")]
    id: Option<String>,
    #[serde(rename = "@profiles")]
    profiles: ListOfProfiles,
    #[serde(rename = "@type")]
    r#type: Option<PresentationType>,
    #[serde(rename = "@availabilityStartTime")]
    availability_start_time: Option<XsDateTime>, // dynamicの場合必須
    #[serde(rename = "@publishTime")]
    publish_time: Option<XsDateTime>, // dynamicの場合必須
    #[serde(rename = "@availabilityEndTime")]
    availability_end_time: Option<XsDateTime>,
    #[serde(rename = "@mediaPresentationDuration")]
    media_presentation_duration: Option<XsDuration>,
    #[serde(rename = "@minimumUpdatePeriod")]
    minimum_undate_period: Option<XsDuration>,
    #[serde(rename = "@minBufferTime")]
    min_buffer_time: XsDuration,
    #[serde(rename = "@timeShiftBufferDepth")]
    time_shift_buffer_depth: Option<XsDuration>,
    #[serde(rename = "@suggestedPresentationDelay")]
    suggested_presentation_delay: Option<XsDuration>,
    #[serde(rename = "@maxSegmentDuration")]
    max_segment_duration: Option<XsDuration>,
    #[serde(rename = "@maxSubsegmentDuration")]
    max_subsegment_duration: Option<XsDuration>,
    #[serde(rename = "ProgramInformation")]
    program_information: Option<Vec<ProgramInformation>>,
    #[serde(rename = "BaseURL")]
    base_url: Option<Vec<BaseURL>>,
    #[serde(rename = "Location")]
    location: Option<Vec<AnyUri>>,
    #[serde(rename = "PatchLocation")]
    patch_location: Option<Vec<PatchLocation>>,
    #[serde(rename = "ServiceDescription")]
    service_description: Option<Vec<ServiceDescription>>,
    #[serde(rename = "InitializationSet")]
    initialization_set: Option<Vec<InitializationSet>>,
    #[serde(rename = "InitializationGroup")]
    initialization_group: Option<Vec<UIntVWithID>>,
    #[serde(rename = "InitializationPresentation")]
    initialization_presentation: Option<Vec<UIntVWithID>>,
    #[serde(rename = "ContentProtection")]
    content_protection: Option<Vec<ContentProtection>>,
    #[serde(rename = "Period")]
    period: Vec<Period>,
    #[serde(rename = "Metrics")]
    metrics: Option<Vec<Metrics>>,
    #[serde(rename = "EssentialProperty")]
    essential_property: Option<Vec<Descriptor>>,
    #[serde(rename = "SupplementalProperty")]
    supplemental_property: Option<Vec<Descriptor>>,
    #[serde(rename = "UTCTiming")]
    utc_timing: Option<Vec<Descriptor>>,
    #[serde(rename = "LeapSecondInformation")]
    leap_second_information: Option<LeapSecondInformation>,
}
