use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::definition::*;
use crate::element::*;
use crate::types::*;

use crate::element::period::Period;

#[skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Builder)]
#[builder(
    setter(into, strip_option),
    default,
    build_fn(validate = "Self::validate")
)]
pub struct MPD {
    #[serde(rename = "@xmlns")]
    #[builder(default = "Some(MPD_NAMESPACE.into())")]
    xmlns: Option<String>,
    #[serde(rename = "@xmlns:xsi")]
    #[builder(default = "Some(MPD_SCHEMA_INSTANCE.into())")]
    xmlns_xsi: Option<String>,
    #[serde(rename = "@xmlns:ext")]
    xmlns_ext: Option<String>,
    #[serde(rename = "@xmlns:xlink")]
    xmlns_xlink: Option<String>,
    #[serde(rename = "@xmlns:cenc")]
    xmlns_cenc: Option<String>,
    #[serde(rename = "@xmlns:dvb")]
    xmlns_dvb: Option<String>,
    #[serde(rename = "@xmlns:scte35")]
    xmlns_scte35: Option<String>,
    #[serde(rename = "@xmlns:scte214")]
    xmlns_scte214: Option<String>,
    #[serde(rename = "@xsi:schemaLocation")]
    #[builder(default = "Some(vec![MPD_NAMESPACE, MPD_SCHEMA_FILE].into())")]
    xsi_schema_location: Option<StringVector>,
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
    location: Option<Vec<XsAnyURI>>,
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

impl NeedValidater for MPDBuilder {
    fn validate(&self) -> Result<(), String> {
        if !self
            .profiles
            .as_ref()
            .is_some_and(|profiles| !profiles.is_empty())
        {
            return Err("MPD must be set profiles.".to_string());
        }

        if self
            .r#type
            .as_ref()
            .is_some_and(|typ| typ == &Some(PresentationType::Dynamic))
        {
            if self.availability_start_time.is_none() || self.publish_time.is_none() {
                return Err("For @type='dynamic', @availabilityStartTime and @publishTime attribute shall be present".to_string());
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_element_mpd_valid() {
        assert!(MPDBuilder::default()
            .profiles(vec![Profile::Full])
            .build()
            .is_ok());
        assert!(MPDBuilder::default()
            .profiles(vec![Profile::Full])
            .r#type(PresentationType::Dynamic)
            .availability_start_time(chrono::Utc::now())
            .publish_time(chrono::Utc::now())
            .build()
            .is_ok());
    }

    #[test]
    fn test_element_mpd_invalid() {
        assert!(MPDBuilder::default().build().is_err());
        assert!(MPDBuilder::default()
            .profiles(vec![Profile::Full])
            .r#type(PresentationType::Dynamic)
            .build()
            .is_err());
        assert!(MPDBuilder::default()
            .profiles(vec![Profile::Full])
            .r#type(PresentationType::Dynamic)
            .availability_start_time(chrono::Utc::now())
            .build()
            .is_err());
        assert!(MPDBuilder::default()
            .profiles(vec![Profile::Full])
            .r#type(PresentationType::Dynamic)
            .publish_time(chrono::Utc::now())
            .build()
            .is_err());
    }
}
