use std::ops::Deref;

use num::BigInt;
use regex::Regex;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct XsDuration(iso8601::Duration);

impl Deref for XsDuration {
    type Target = iso8601::Duration;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<&[u8]> for XsDuration {
    fn from(value: &[u8]) -> Self {
        Self(
            iso8601::parsers::parse_duration(value)
                .and_then(|(_, duration)| Ok(duration))
                .unwrap_or_default(),
        )
    }
}

impl From<String> for XsDuration {
    fn from(value: String) -> Self {
        Self::from(value.as_bytes())
    }
}

impl From<&str> for XsDuration {
    fn from(value: &str) -> Self {
        Self::from(value.as_bytes())
    }
}

impl Serialize for XsDuration {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.0.to_string())
    }
}

impl<'de> Deserialize<'de> for XsDuration {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let duration = s
            .parse::<iso8601::Duration>()
            .map_err(serde::de::Error::custom)?;
        Ok(XsDuration(duration))
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct XsInteger(BigInt);

impl Deref for XsInteger {
    type Target = BigInt;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<BigInt> for XsInteger {
    fn from(value: BigInt) -> Self {
        Self(value)
    }
}

impl From<i32> for XsInteger {
    fn from(value: i32) -> Self {
        Self(value.into())
    }
}

impl From<i64> for XsInteger {
    fn from(value: i64) -> Self {
        Self(value.into())
    }
}

impl Serialize for XsInteger {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.0.to_string())
    }
}

impl<'de> Deserialize<'de> for XsInteger {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let big_int = s.parse::<BigInt>().map_err(serde::de::Error::custom)?;
        Ok(XsInteger(big_int))
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct SingleRFC7233Range {
    pub start: Option<u64>,
    pub end: Option<u64>,
}

impl From<(Option<u64>, Option<u64>)> for SingleRFC7233Range {
    fn from(value: (Option<u64>, Option<u64>)) -> Self {
        Self {
            start: value.0,
            end: value.1,
        }
    }
}

impl Serialize for SingleRFC7233Range {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let start_str = self.start.map_or("".to_string(), |s| s.to_string());
        let end_str = self.end.map_or("".to_string(), |e| e.to_string());
        let s = if self.end.is_some() || !start_str.is_empty() {
            format!("{}-{}", start_str, end_str)
        } else {
            "".to_string()
        };
        serializer.serialize_str(&s)
    }
}

impl<'de> Deserialize<'de> for SingleRFC7233Range {
    fn deserialize<D>(deserializer: D) -> Result<SingleRFC7233Range, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let re = Regex::new(r"^([0-9]*)(-([0-9]*))?$").unwrap();
        if let Some(caps) = re.captures(&s) {
            let start = caps.get(1).and_then(|m| {
                if m.as_str().is_empty() {
                    None
                } else {
                    m.as_str().parse::<u64>().ok()
                }
            });
            let end = caps.get(3).and_then(|m| {
                if m.as_str().is_empty() {
                    None
                } else {
                    m.as_str().parse::<u64>().ok()
                }
            });
            Ok(SingleRFC7233Range { start, end })
        } else {
            Err(serde::de::Error::custom(
                "Invalid format for SingleRFC7233Range",
            ))
        }
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct XsAnyUri(String);

impl Deref for XsAnyUri {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Url {
    #[serde(rename = "@sourceURL", skip_serializing_if = "Option::is_none")]
    pub source_url: Option<XsAnyUri>,
    #[serde(rename = "@range", skip_serializing_if = "Option::is_none")]
    pub range: Option<SingleRFC7233Range>,
}

impl From<(Option<String>, (Option<u64>, Option<u64>))> for Url {
    fn from(value: (Option<String>, (Option<u64>, Option<u64>))) -> Self {
        Self {
            source_url: value.0.and_then(|s| Some(XsAnyUri(s))),
            range: Some(SingleRFC7233Range::from(value.1)),
        }
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename = "FCS")]
pub struct Fcs {
    #[serde(rename = "@t")]
    pub start_time: u64,
    #[serde(rename = "@d", skip_serializing_if = "Option::is_none")]
    pub duration: Option<u64>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename = "FailoverContent")]
pub struct FailoverContent {
    #[serde(rename = "@valid")]
    pub valid: Option<bool>,
    #[serde(rename = "FCS", skip_serializing_if = "Vec::is_empty")]
    pub fcs_list: Vec<Fcs>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename = "SegmentURL")]
pub struct SegmentUrl {
    #[serde(rename = "@media")]
    media: Option<XsAnyUri>,
    #[serde(rename = "@mediaRange")]
    media_range: Option<SingleRFC7233Range>,
    #[serde(rename = "@index")]
    index: Option<XsAnyUri>,
    #[serde(rename = "@indexRange")]
    index_range: Option<SingleRFC7233Range>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_types_xs_integer_serde() {
        let value = 10000;
        let xs_integer = XsInteger::from(value);
        let ser = serde_plain::to_string(&xs_integer).unwrap();

        assert!(ser == value.to_string());

        let der = serde_plain::from_str::<XsInteger>(&ser);

        assert!(der.is_ok_and(|val| val == xs_integer));
    }

    #[test]
    fn test_types_xs_duration_serde() {
        let value = "foo";
        let xs_duration = XsDuration::from(value);
        assert!(*xs_duration == iso8601::Duration::default());

        let value = "PT3H11M53S";
        let xs_duration = XsDuration::from(value);
        let ser = serde_plain::to_string(&xs_duration).unwrap();

        assert!(&ser == value);

        let der = serde_plain::from_str::<XsDuration>(&ser);

        assert!(der.is_ok_and(|val| val == xs_duration));
    }

    #[test]
    fn test_types_single_range_type_serde_full() {
        let plain = "100-200";
        let result = serde_plain::from_str::<SingleRFC7233Range>(&plain).unwrap();

        assert_eq!(
            result,
            SingleRFC7233Range {
                start: Some(100),
                end: Some(200)
            }
        );

        let ser = serde_plain::to_string(&result).unwrap();

        assert_eq!(plain, ser.as_str());
    }

    #[test]
    fn test_types_single_range_type_serde_start_only() {
        let plain = "100-";
        let result = serde_plain::from_str::<SingleRFC7233Range>(&plain).unwrap();

        assert_eq!(
            result,
            SingleRFC7233Range {
                start: Some(100),
                end: None
            }
        );

        let ser = serde_plain::to_string(&result).unwrap();

        assert_eq!(plain, ser.as_str());
    }

    #[test]
    fn test_types_single_range_type_serde_end_only() {
        let plain = "-200";
        let result = serde_plain::from_str::<SingleRFC7233Range>(&plain).unwrap();

        assert_eq!(
            result,
            SingleRFC7233Range {
                start: None,
                end: Some(200)
            }
        );

        let ser = serde_plain::to_string(&result).unwrap();

        assert_eq!(plain, ser.as_str());
    }

    #[test]
    fn test_types_single_range_type_serde_empty() {
        let plain = "";
        let result = serde_plain::from_str::<SingleRFC7233Range>(&plain).unwrap();

        assert_eq!(
            result,
            SingleRFC7233Range {
                start: None,
                end: None
            }
        );

        let ser = serde_plain::to_string(&result).unwrap();

        assert_eq!(plain, ser.as_str());
    }

    #[test]
    fn test_types_single_range_type_invalid_format() {
        let plain = "abc-xyz";
        let result = serde_plain::from_str::<SingleRFC7233Range>(&plain);

        assert!(result.is_err());
    }

    #[test]
    fn test_types_url_type_serde() {
        let xml = r#"<Url sourceURL="http://example.com/video.mp4" range="100-200"/>"#;

        let ret = quick_xml::de::from_str::<Url>(&xml).unwrap();

        assert_eq!(
            ret,
            Url {
                source_url: Some(XsAnyUri("http://example.com/video.mp4".to_string())),
                range: Some(SingleRFC7233Range {
                    start: Some(100),
                    end: Some(200)
                })
            }
        );

        let mut se = String::new();
        let ser = quick_xml::se::Serializer::new(&mut se);
        ret.serialize(ser).unwrap();

        assert_eq!(xml, se.as_str());
    }

    #[test]
    fn test_types_failover_content_type_serde() {
        let xml = r#"<FailoverContent valid="true">
  <FCS t="1625152800" d="3600"/>
  <FCS t="1625156400"/>
</FailoverContent>"#;

        let ret = quick_xml::de::from_str::<FailoverContent>(&xml).unwrap();

        assert_eq!(
            ret,
            FailoverContent {
                valid: Some(true),
                fcs_list: vec![
                    Fcs {
                        start_time: 1625152800,
                        duration: Some(3600)
                    },
                    Fcs {
                        start_time: 1625156400,
                        duration: None
                    }
                ]
            }
        );

        let mut se = String::new();
        let mut ser = quick_xml::se::Serializer::new(&mut se);
        ser.indent(' ', 2);
        ret.serialize(ser).unwrap();

        assert_eq!(xml, se.as_str());
    }
}
