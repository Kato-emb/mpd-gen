use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::element::*;
use crate::types::*;

use super::{
    adapt::AdaptationSet,
    segment::{SegmentBase, SegmentList, SegmentTemplate},
};

#[skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Builder)]
#[builder(setter(into, strip_option), default, build_fn(error = "MpdError"))]
pub struct Period {
    #[serde(rename = "@xlink:href")]
    href: Option<String>,
    #[serde(rename = "@xlink:actuate")]
    actuate: Option<XLinkActure>,
    #[serde(rename = "@id")]
    id: Option<u32>,
    #[serde(rename = "@start")]
    start: Option<XsDuration>,
    #[serde(rename = "@duration")]
    duration: Option<XsDuration>,
    #[serde(rename = "@bitstreamSwitching")]
    bitstream_switching: Option<bool>,
    #[serde(rename = "BaseURL")]
    base_url: Option<Vec<BaseURL>>,
    #[serde(rename = "SegmentBase")]
    segment_base: Option<SegmentBase>,
    #[serde(rename = "SegmentList")]
    segment_list: Option<SegmentList>,
    #[serde(rename = "SegmentTemplate")]
    segment_template: Option<SegmentTemplate>,
    asset_identifier: Option<Descriptor>,
    #[serde(rename = "EventStream")]
    event_stream: Option<Vec<EventStream>>,
    #[serde(rename = "ServiceDescription")]
    service_description: Option<Vec<ServiceDescription>>,
    #[serde(rename = "ContentProtection")]
    content_protection: Option<Vec<ContentProtection>>,
    #[serde(rename = "AdaptationSet")]
    adaptation_set: Option<Vec<AdaptationSet>>,
    #[serde(rename = "Subset")]
    subset: Option<Vec<Subset>>,
    #[serde(rename = "SupplementalProperty")]
    supplemental_property: Option<Vec<Descriptor>>,
    #[serde(rename = "EmptyAdaptationSet")]
    empty_adaptation_set: Option<Vec<AdaptationSet>>,
    #[serde(rename = "GroupLabel")]
    group_lavel: Option<Vec<GroupLavel>>,
    #[serde(rename = "Preselection")]
    preselection: Option<Vec<Preselection>>,
}
