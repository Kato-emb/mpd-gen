use std::str::FromStr;

use strum_macros::{Display, EnumString};

use crate::{define_regex, entity::*, error::MpdError, Result};

pub const XML_DECLARATION: &str = r#"<?xml version="1.0" encoding="UTF-8"?>"#;

pub const MPD_NAMESPACE: &str = "urn:mpeg:dash:schema:mpd:2011";
pub const XML_LINKING_LANGUAGE: &str = "http://www.w3.org/1999/xlink";
pub const MPD_SCHEMA_FILE: &str = "DASH-MPD.xsd";
pub const MPD_SCHEMA_INSTANCE: &str = "http://www.w3.org/2001/XMLSchema-instance";
pub const DASH_DVB_EXTENTION: &str = "urn:dvb:dash-extentions:2014-1";

pub const ROLE_SCHEME: &str = "urn:mpeg:dash:role:2011";

#[derive(Debug, Clone, PartialEq, Eq, Hash, EnumString, Display)]
pub enum PeriodSignalling {
    #[strum(serialize = "urn:mpeg:dash:period-continuity:2015")]
    Continuity,
    #[strum(serialize = "urn:mpeg:dash:period-connectivity:2015")]
    Connectivity,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Display)]
pub enum Profile {
    #[strum(serialize = "urn:mpeg:dash:profile:full:2011")]
    Full,
    #[strum(serialize = "urn:mpeg:dash:profile:isoff-on-demand:2011")]
    IsoOnDemand,
    #[strum(serialize = "urn:mpeg:dash:profile:isoff-live:2011")]
    IsoLive,
    #[strum(serialize = "urn:mpeg:dash:profile:isoff-main:2011")]
    IsoMain,
    #[strum(serialize = "urn:mpeg:dash:profile:mp2t-main:2011")]
    Mp2tMain,
    #[strum(serialize = "urn:mpeg:dash:profile:mp2t-simple:2011")]
    Mp2tSimple,
    #[strum(serialize = "urn:mpeg:dash:profile:isoff-ext-live:2014")]
    IsoExtLive,
    #[strum(serialize = "urn:mpeg:dash:profile:isoff-ext-on-demand:2014")]
    IsoExtOnDemand,
    #[strum(serialize = "urn:mpeg:dash:profile:isoff-common:2014")]
    IsoCommon,
    #[strum(serialize = "urn:mpeg:dash:profile:isoff-broadcast:2015")]
    IsoBroadcast,
    #[strum(serialize = "urn:mpeg:dash:profile:cmaf:2019")]
    Cmaf,
    #[strum(serialize = "urn:mpeg:dash:profile:cmaf-extended:2019")]
    CmafExt,
    #[strum(serialize = "{0}")]
    Other(String),
}

impl FromStr for Profile {
    type Err = MpdError;

    fn from_str(s: &str) -> Result<Self> {
        if PATTERN_PROFILE.is_match(s) {
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

#[derive(Debug, Clone, PartialEq, Eq, Hash, EnumString, Display)]
pub enum Identifier {
    #[strum(serialize = "$RepresentationID$")]
    RepresentationID,
    #[strum(serialize = "$Number$")]
    Number,
    #[strum(serialize = "$Bandwidth$")]
    Bandwidth,
    #[strum(serialize = "$Time$")]
    Time,
    #[strum(serialize = "$SubNumber$")]
    SubNumber,
}

define_regex!(
    PATTERN_URL_TEMPLATE,
    r"(?P<identifier>\$RepresentationID\$|\$Number\$|\$Bandwidth\$|\$Time\$|\$SubNumber\$)",
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_definition_period_signalling_parse() {
        let input = "urn:mpeg:dash:period-continuity:2015";
        let signalling = PeriodSignalling::from_str(&input).unwrap();
        assert_eq!(&signalling.to_string(), input);
    }

    #[test]
    fn test_definition_profile_parse() {
        let input = "urn:mpeg:dash:profile:full:2011";
        let profile = Profile::from_str(&input).unwrap();
        assert_eq!(profile, Profile::Full);

        let input = "https://example.com";
        let profile = Profile::from_str(&input).unwrap();
        assert_eq!(profile, Profile::Other(input.to_string()));
    }

    #[test]
    fn test_definition_url_template_pattern_matching() {
        let input = format!(
            "{}/{}/{}.cmfv",
            Identifier::RepresentationID,
            Identifier::Bandwidth,
            Identifier::Time
        );

        let replace_str = PATTERN_URL_TEMPLATE
            .replace_all(&input, |caps: &regex::Captures| {
                if let Some(m) = caps.name("identifier") {
                    match Identifier::from_str(m.as_str()) {
                        Ok(Identifier::RepresentationID) => return "720p".to_string(),
                        Ok(Identifier::Bandwidth) => return "2000000".to_string(),
                        Ok(Identifier::Time) => return "1000".to_string(),
                        _ => {}
                    }
                }

                caps.get(0).unwrap().as_str().to_string()
            })
            .to_string();

        assert_eq!(replace_str, "720p/2000000/1000.cmfv".to_string())
    }
}
