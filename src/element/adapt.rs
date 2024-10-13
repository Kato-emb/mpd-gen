use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::element::*;
use crate::types::*;

use super::{
    repr::Representation,
    segment::{SegmentBase, SegmentList, SegmentTemplate},
};

#[skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct AdaptationSet {
    #[serde(rename = "@xlink:href")]
    href: Option<String>,
    #[serde(rename = "@xlink:actuate")]
    actuate: Option<XLinkActure>,
    #[serde(rename = "@id")]
    id: Option<u32>,
    #[serde(rename = "@group")]
    group: Option<u32>,
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
    #[serde(rename = "@lang")]
    lang: Option<XsLanguage>,
    #[serde(rename = "@contentType")]
    content_type: Option<ContentType>,
    #[serde(rename = "@par")]
    par: Option<Ratio>,
    #[serde(rename = "@minBandwidth")]
    min_bandwidth: Option<u32>,
    #[serde(rename = "@maxBandwidth")]
    max_bandwidth: Option<u32>,
    #[serde(rename = "@minWidth")]
    min_width: Option<u32>,
    #[serde(rename = "@maxWidth")]
    max_width: Option<u32>,
    #[serde(rename = "@minHeight")]
    min_height: Option<u32>,
    #[serde(rename = "@maxHeight")]
    max_height: Option<u32>,
    #[serde(rename = "@minFrameRate")]
    min_framerate: Option<FrameRate>,
    #[serde(rename = "@maxFrameRate")]
    max_framerate: Option<FrameRate>,
    #[serde(rename = "@segmentAlignment")]
    segment_alignment: Option<bool>,
    #[serde(rename = "@bitstreamSwitching")]
    bitstream_switching: Option<bool>,
    #[serde(rename = "@subsegmentAlignment")]
    subsegment_alignment: Option<bool>,
    #[serde(rename = "@subsegmentStartsWithSAP")]
    subsegment_starts_with_sap: Option<StreamAccessPoint>,
    #[serde(rename = "@initializationSetRef")]
    initialization_set_ref: Option<UIntVector>,
    #[serde(rename = "@initializationPrincipal")]
    initialization_principal: Option<XsAnyURI>,
    #[serde(rename = "Accessibility")]
    accessibility: Option<Vec<Descriptor>>,
    #[serde(rename = "Role")]
    role: Option<Vec<Descriptor>>,
    #[serde(rename = "Rating")]
    rating: Option<Vec<Descriptor>>,
    #[serde(rename = "Viewpoint")]
    viewpoint: Option<Vec<Descriptor>>,
    #[serde(rename = "ContentComponent")]
    content_component: Option<Vec<ContentComponent>>,
    #[serde(rename = "BaseURL")]
    base_url: Option<Vec<BaseURL>>,
    #[serde(rename = "SegmentBase")]
    segment_base: Option<SegmentBase>,
    #[serde(rename = "SegmentList")]
    segment_list: Option<SegmentList>,
    #[serde(rename = "SegmentTemplate")]
    segment_template: Option<SegmentTemplate>,
    #[serde(rename = "Representation")]
    representation: Option<Vec<Representation>>,
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::{
        element::{repr::RepresentationBuilder, segment::SegmentTemplateBuilder},
        SegmentBuilder, SegmentTimelineBuilder,
    };

    use super::*;

    #[test]
    fn test_element_adaptation_set() {
        let segment_timeline = SegmentTimelineBuilder::default()
            .segment(
                SegmentBuilder::default()
                    .duration(15000u64)
                    .start_time(0u64)
                    .repeat_count(10)
                    .build()
                    .unwrap(),
            )
            .build()
            .unwrap();
        let segment_template = SegmentTemplateBuilder::default()
            .initialization_attribute("$RepresentationID$.cmfi")
            .media("$RepresentationID$/$Time$.cmfv")
            .timescale(3000u32)
            .segment_timeline(segment_timeline)
            .build()
            .unwrap();

        let representation = RepresentationBuilder::default()
            .id(NoWhitespace::from_str("720p").unwrap())
            .codecs(Codecs::from_str("avc1.4d0028").unwrap())
            .bandwidth(4_000_000u32)
            .width(1280u32)
            .height(720u32)
            .build()
            .unwrap();

        let role = Descriptor::from((
            "urn:mpeg:dash:role:2011".to_string(),
            (Some("main".to_string()), None),
        ));

        let adapt = AdaptationSetBuilder::default()
            .content_type(ContentType::Video)
            .segment_alignment(true)
            .mime_type("video/mp4")
            .start_with_sap(StreamAccessPoint::Type1)
            .role(vec![role])
            .segment_template(segment_template)
            .representation(vec![representation])
            .build()
            .unwrap();

        let mut xml = String::new();
        let mut ser = quick_xml::se::Serializer::new(&mut xml);
        ser.indent(' ', 2);
        adapt.serialize(ser).unwrap();

        println!("{xml}");
    }
}
