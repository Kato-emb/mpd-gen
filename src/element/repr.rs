use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::types::{AspectRatio, FrameRate, ListOfFourCC, ListOfProfiles, UIntVector};
use crate::{
    scheme::Profile,
    types::{StringNoWhitespace, StringVector},
};

#[skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct Representation {
    #[serde(rename = "@id")]
    id: StringNoWhitespace,
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
    audio_sampling_rate: Option<UIntVector>,
    mime_type: Option,
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn test_element_representation() {
        let repr = RepresentationBuilder::default()
            .id(StringNoWhitespace::from_str("aaaaaa").unwrap())
            .bandwidth(2_000_000u32)
            .dependency_id(["a".to_string(), "b".to_string()].as_slice())
            .association_type([0x54534554, 0x4D4A5047].as_slice())
            .build()
            .unwrap();

        let mut xml = String::new();
        let mut ser = quick_xml::se::Serializer::new(&mut xml);
        ser.indent(' ', 2);
        repr.serialize(ser).unwrap();

        println!("{}", xml);
    }
}
