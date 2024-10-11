use std::{fmt, str::FromStr};

use serde::{Deserialize, Serialize};

use crate::{
    entity::{PATTERN_URL, PATTERN_URN},
    error::MpdError,
    Result,
};

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum Profile {
    #[default]
    Full,
    IsoOnDemand,
    IsoLive,
    IsoMain,
    Mp2tMain,
    Mp2tSimple,
    IsoExtLive,
    IsoExtOnDemand,
    IsoCommon,
    IsoBroadcast,
    Cmaf,
    CmafExt,
    Other(String),
}

impl FromStr for Profile {
    type Err = MpdError;

    fn from_str(s: &str) -> Result<Self> {
        if PATTERN_URN.is_match(s) || PATTERN_URL.is_match(s) {
            Ok(match s {
                "urn:mpeg:dash:profile:full:2011" => Profile::Full,
                "urn:mpeg:dash:profile:isoff-on-demand:2011" => Profile::IsoOnDemand,
                "urn:mpeg:dash:profile:isoff-live:2011" => Profile::IsoLive,
                "urn:mpeg:dash:profile:isoff-main:2011" => Profile::IsoMain,
                "urn:mpeg:dash:profile:mp2t-main:2011" => Profile::Mp2tMain,
                "urn:mpeg:dash:profile:mp2t-simple:2011" => Profile::Mp2tSimple,
                "urn:mpeg:dash:profile:isoff-ext-live:2014" => Profile::IsoExtLive,
                "urn:mpeg:dash:profile:isoff-ext-on-demand:2014" => Profile::IsoExtOnDemand,
                "urn:mpeg:dash:profile:isoff-common:2014" => Profile::IsoCommon,
                "urn:mpeg:dash:profile:isoff-broadcast:2015" => Profile::IsoBroadcast,
                "urn:mpeg:dash:profile:cmaf:2019" => Profile::Cmaf,
                "urn:mpeg:dash:profile:cmaf-extended:2019" => Profile::CmafExt,
                other => Profile::Other(other.to_string()),
            })
        } else {
            Err(MpdError::UnmatchedPattern)
        }
    }
}

impl fmt::Display for Profile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Profile::Full => "urn:mpeg:dash:profile:full:2011",
            Profile::IsoOnDemand => "urn:mpeg:dash:profile:isoff-on-demand:2011",
            Profile::IsoLive => "urn:mpeg:dash:profile:isoff-live:2011",
            Profile::IsoMain => "urn:mpeg:dash:profile:isoff-main:2011",
            Profile::Mp2tMain => "urn:mpeg:dash:profile:mp2t-main:2011",
            Profile::Mp2tSimple => "urn:mpeg:dash:profile:mp2t-simple:2011",
            Profile::IsoExtLive => "urn:mpeg:dash:profile:isoff-ext-live:2014",
            Profile::IsoExtOnDemand => "urn:mpeg:dash:profile:isoff-ext-on-demand:2014",
            Profile::IsoCommon => "urn:mpeg:dash:profile:isoff-common:2014",
            Profile::IsoBroadcast => "urn:mpeg:dash:profile:isoff-broadcast:2015",
            Profile::Cmaf => "urn:mpeg:dash:profile:cmaf:2019",
            Profile::CmafExt => "urn:mpeg:dash:profile:cmaf-extended:2019",
            Profile::Other(profile) => &profile,
        };

        write!(f, "{s}")
    }
}
