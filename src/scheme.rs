use std::fmt;

use serde::{Deserialize, Serialize};

use crate::{error::MpdError, Result};

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
    GppDash10,
    HbbTV20Dash,
    HbbTV15Dash,
}

impl std::str::FromStr for Profile {
    type Err = MpdError;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "urn:mpeg:dash:profile:full:2011" => Ok(Profile::Full),
            "urn:mpeg:dash:profile:isoff-on-demand:2011" => Ok(Profile::IsoOnDemand),
            "urn:mpeg:dash:profile:isoff-live:2011" => Ok(Profile::IsoLive),
            "urn:mpeg:dash:profile:isoff-main:2011" => Ok(Profile::IsoMain),
            "urn:mpeg:dash:profile:mp2t-main:2011" => Ok(Profile::Mp2tMain),
            "urn:mpeg:dash:profile:mp2t-simple:2011" => Ok(Profile::Mp2tSimple),
            "urn:mpeg:dash:profile:isoff-ext-live:2014" => Ok(Profile::IsoExtLive),
            "urn:mpeg:dash:profile:isoff-ext-on-demand:2014" => Ok(Profile::IsoExtOnDemand),
            "urn:mpeg:dash:profile:isoff-common:2014" => Ok(Profile::IsoCommon),
            "urn:mpeg:dash:profile:isoff-broadcast:2015" => Ok(Profile::IsoBroadcast),
            "urn:mpeg:dash:profile:cmaf:2019" => Ok(Profile::Cmaf),
            "urn:mpeg:dash:profile:cmaf-extended:2019" => Ok(Profile::CmafExt),
            "urn:3GPP:PSS:profile:DASH10" => Ok(Profile::GppDash10),
            "urn:dvb:dash:profile:dvb-dash:2014" => Ok(Profile::HbbTV20Dash),
            "urn:hbbtv:dash:profile:isoff-live:2012" => Ok(Profile::HbbTV15Dash),
            _other => Err(MpdError::InvalidData("Unsupported profile uri.")),
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
            Profile::GppDash10 => "urn:3GPP:PSS:profile:DASH10",
            Profile::HbbTV20Dash => "urn:dvb:dash:profile:dvb-dash:2014",
            Profile::HbbTV15Dash => "urn:hbbtv:dash:profile:isoff-live:2012",
        };

        write!(f, "{s}")
    }
}
