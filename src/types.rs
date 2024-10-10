use core::str;
use std::{fmt, ops::Deref, result, str::FromStr};

use num::{integer::gcd, BigInt};
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_with::{DeserializeFromStr, SerializeDisplay};

use crate::{error::MpdError, scheme::Profile, Result};

#[derive(Debug, Default, Clone, SerializeDisplay, DeserializeFromStr, PartialEq, Eq, Hash)]
pub struct StringNoWhitespace {
    value: String,
}

impl fmt::Display for StringNoWhitespace {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl FromStr for StringNoWhitespace {
    type Err = MpdError;

    fn from_str(s: &str) -> result::Result<Self, Self::Err> {
        let re = Regex::new(r"^[^\r\n\t \p{Z}]*$").unwrap();
        if !re.is_match(s) {
            return Err(MpdError::UnmatchedPattern);
        }

        Ok(Self {
            value: s.to_string(),
        })
    }
}

#[derive(Default, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct FourCC {
    value: [u8; 4],
}

impl Deref for FourCC {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        str::from_utf8(&self.value).unwrap()
    }
}

impl FromStr for FourCC {
    type Err = MpdError;

    fn from_str(s: &str) -> Result<Self> {
        if let [a, b, c, d] = s.as_bytes() {
            Ok(Self {
                value: [*a, *b, *c, *d],
            })
        } else {
            Err(MpdError::UnmatchedPattern)
        }
    }
}

impl From<u32> for FourCC {
    fn from(number: u32) -> Self {
        FourCC {
            value: number.to_be_bytes(),
        }
    }
}

impl From<FourCC> for u32 {
    fn from(fourcc: FourCC) -> u32 {
        (&fourcc).into()
    }
}

impl From<&FourCC> for u32 {
    fn from(fourcc: &FourCC) -> u32 {
        u32::from_be_bytes(fourcc.value)
    }
}

impl From<[u8; 4]> for FourCC {
    fn from(value: [u8; 4]) -> FourCC {
        FourCC { value }
    }
}

impl fmt::Debug for FourCC {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let code: u32 = self.into();
        let string = String::from_utf8_lossy(&self.value[..]);
        write!(f, "{string} / {code:#010X}")
    }
}

impl fmt::Display for FourCC {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", String::from_utf8_lossy(&self.value[..]))
    }
}

#[derive(Debug, Default, Clone, SerializeDisplay, DeserializeFromStr, PartialEq, Eq, Hash)]
pub struct WhitespaceSeparatedList<T: fmt::Display + FromStr> {
    value: Vec<T>,
}

impl<S, T> From<Vec<S>> for WhitespaceSeparatedList<T>
where
    S: Into<T>,
    T: fmt::Display + FromStr,
{
    fn from(value: Vec<S>) -> Self {
        Self {
            value: value.into_iter().map(|item| item.into()).collect(),
        }
    }
}

impl<S, T> From<&[S]> for WhitespaceSeparatedList<T>
where
    S: Into<T> + Clone,
    T: fmt::Display + FromStr,
{
    fn from(value: &[S]) -> Self {
        Self {
            value: value.into_iter().map(|item| item.clone().into()).collect(),
        }
    }
}

impl<T: fmt::Display + FromStr> fmt::Display for WhitespaceSeparatedList<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let joined = self
            .value
            .iter()
            .map(|item| item.to_string())
            .collect::<Vec<String>>()
            .join(" ");
        write!(f, "{joined}")
    }
}

impl<T: fmt::Display + FromStr> FromStr for WhitespaceSeparatedList<T> {
    type Err = MpdError;

    fn from_str(s: &str) -> Result<Self> {
        let items = s
            .split_whitespace()
            .map(|item| {
                T::from_str(item).map_err(|_| MpdError::InvalidData("Failed to parse from str"))
            })
            .collect::<Result<Vec<T>>>()?;
        Ok(Self { value: items })
    }
}

pub type UIntVector = WhitespaceSeparatedList<u32>;
pub type StringVector = WhitespaceSeparatedList<String>;
pub type ListOfFourCC = WhitespaceSeparatedList<FourCC>;

#[derive(Debug, Default, Clone, SerializeDisplay, DeserializeFromStr, PartialEq, Eq, Hash)]
pub struct ListOfProfiles {
    value: Vec<Profile>,
}

impl fmt::Display for ListOfProfiles {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let joined = self
            .value
            .iter()
            .map(|item| item.to_string())
            .collect::<Vec<String>>()
            .join(",");
        write!(f, "{joined}")
    }
}

impl FromStr for ListOfProfiles {
    type Err = MpdError;

    fn from_str(s: &str) -> result::Result<Self, Self::Err> {
        let items = s
            .split(",")
            .map(|item| Profile::from_str(item))
            .collect::<Result<Vec<Profile>>>()?;

        Ok(Self { value: items })
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AspectRatio {
    horizontal: u64,
    vertical: u64,
}

impl From<(u64, u64)> for AspectRatio {
    fn from(value: (u64, u64)) -> Self {
        let (numer, denom) = value;
        let divisor = gcd(numer, denom);

        Self {
            horizontal: numer / divisor,
            vertical: denom / divisor,
        }
    }
}

impl fmt::Display for AspectRatio {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.horizontal, self.vertical)
    }
}

impl FromStr for AspectRatio {
    type Err = MpdError;

    fn from_str(s: &str) -> Result<Self> {
        let parts: Vec<&str> = s.split(':').collect();

        if parts.len() != 2 {
            return Err(MpdError::InvalidData("Aspect ratio format `_:_`"));
        }

        let horizontal = parts[0].parse::<u64>()?;
        let vertical = parts[1].parse::<u64>()?;

        Ok(Self {
            horizontal,
            vertical,
        })
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct FrameRate {
    frame: u64,
    denom: Option<u64>,
}

impl fmt::Display for FrameRate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}/{}", self.frame, self.denom.unwrap_or(1))
    }
}

impl FromStr for FrameRate {
    type Err = MpdError;

    fn from_str(s: &str) -> Result<Self> {
        let re = Regex::new(r"^[0-9]+(/[1-9][0-9]*)?$").unwrap();
        if !re.is_match(s) {
            return Err(MpdError::UnmatchedPattern);
        }

        let parts: Vec<&str> = s.split('/').collect();
        let frame = parts[0].parse::<u64>()?;
        let denom = if let Some(s) = parts.get(1) {
            Some(s.parse::<u64>()?)
        } else {
            None
        };

        Ok(Self { frame, denom })
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum RFC6838ContentType {
    Text,
    Image,
    Audio,
    #[default]
    Video,
    Application,
    Font,
}
/// SAP
#[repr(u8)]
#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum StreamAccessPoint {
    #[default]
    /// Closed GoP random access point
    ///
    /// Tept = Tdec = Tsap = Tptf
    Type1 = 1,
    /// Closed GoP random access point
    ///
    /// Tept = Tdec = Tsap < Tptf
    Type2 = 2,
    /// Open GoP random access point
    ///
    /// Tept < Tdec = Tsap <= Tptf
    Type3 = 3,
    /// Gradual Decoding Refresh (GDR) random access point
    ///
    /// Tept <= Tptf < Tdec = Tsap
    Type4 = 4,
    /// Tept = Tdec < Tsap
    Type5 = 5,
    /// Tept < Tdec < Tsap
    Type6 = 6,
}

impl TryFrom<u8> for StreamAccessPoint {
    type Error = MpdError;

    fn try_from(value: u8) -> result::Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::Type1),
            2 => Ok(Self::Type2),
            3 => Ok(Self::Type3),
            4 => Ok(Self::Type4),
            5 => Ok(Self::Type5),
            6 => Ok(Self::Type6),
            _ => Err(MpdError::InvalidData("SAP values must be 1 to 6")),
        }
    }
}

impl fmt::Display for StreamAccessPoint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", *self as u8)
    }
}

impl FromStr for StreamAccessPoint {
    type Err = MpdError;

    fn from_str(s: &str) -> Result<Self> {
        Ok(StreamAccessPoint::try_from(s.parse::<u8>()?)?)
    }
}

#[derive(Debug, Default, Clone, SerializeDisplay, DeserializeFromStr, PartialEq, Eq)]
pub struct XsDuration {
    value: iso8601::Duration,
}

impl fmt::Display for XsDuration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value.to_string())
    }
}

impl FromStr for XsDuration {
    type Err = MpdError;

    fn from_str(s: &str) -> Result<Self> {
        let value = s
            .parse::<iso8601::Duration>()
            .map_err(|_| MpdError::InvalidData("Failed to parse xs:duration"))?;

        Ok(Self { value })
    }
}

#[derive(Debug, Default, Clone, SerializeDisplay, DeserializeFromStr, PartialEq, Eq, Hash)]
pub struct XsInteger {
    value: BigInt,
}

impl<T> From<T> for XsInteger
where
    T: Into<BigInt>,
{
    fn from(value: T) -> Self {
        Self {
            value: value.into(),
        }
    }
}

impl fmt::Display for XsInteger {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value.to_string())
    }
}

impl FromStr for XsInteger {
    type Err = MpdError;

    fn from_str(s: &str) -> result::Result<Self, Self::Err> {
        let value = s
            .parse::<BigInt>()
            .map_err(|_| MpdError::InvalidData("Failed to parse xs:integer"))?;

        Ok(Self { value })
    }
}

#[derive(Debug, Default, Clone, SerializeDisplay, DeserializeFromStr, PartialEq, Eq, Hash)]
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

impl fmt::Display for SingleRFC7233Range {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let start_str = self.start.map_or("".to_string(), |s| s.to_string());
        let end_str = self.end.map_or("".to_string(), |e| e.to_string());
        let s = if self.end.is_some() || !start_str.is_empty() {
            format!("{}-{}", start_str, end_str)
        } else {
            "".to_string()
        };

        write!(f, "{s}")
    }
}

impl FromStr for SingleRFC7233Range {
    type Err = MpdError;

    fn from_str(s: &str) -> Result<Self> {
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
            Err(MpdError::UnmatchedPattern)
        }
    }
}

type AnyUri = String;

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Url {
    #[serde(rename = "@sourceURL", skip_serializing_if = "Option::is_none")]
    pub source_url: Option<AnyUri>,
    #[serde(rename = "@range", skip_serializing_if = "Option::is_none")]
    pub range: Option<SingleRFC7233Range>,
}

impl From<(Option<String>, (Option<u64>, Option<u64>))> for Url {
    fn from(value: (Option<String>, (Option<u64>, Option<u64>))) -> Self {
        Self {
            source_url: value.0.and_then(|s| Some(s)),
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
    media: Option<AnyUri>,
    #[serde(rename = "@mediaRange")]
    media_range: Option<SingleRFC7233Range>,
    #[serde(rename = "@index")]
    index: Option<AnyUri>,
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
        let ret = XsDuration::from_str(value);
        assert!(ret.is_err());

        let value = "PT3H11M53S";
        let xs_duration = XsDuration::from_str(value).unwrap();
        let ser = serde_plain::to_string(&xs_duration).unwrap();

        assert!(&ser == value);

        let der = serde_plain::from_str::<XsDuration>(&ser);

        assert!(der.is_ok_and(|val| val == xs_duration));
    }

    #[test]
    fn test_types_aspect_ratio() {
        let value = "16:9";
        let ratio_parse = AspectRatio::from_str(&value).unwrap();
        let ratio = AspectRatio::from((1920, 1080));

        assert_eq!(ratio_parse, ratio);

        let ser = ratio.to_string();

        assert_eq!(value, &ser);
    }

    #[test]
    fn test_types_frame_rate() {
        let value = "30";
        let framerate = FrameRate::from_str(&value).unwrap();

        assert_eq!(framerate.frame, 30);
        assert_eq!(framerate.denom, None);

        let ser = framerate.to_string();

        assert_eq!(ser, "30/1".to_string());
    }

    #[test]
    fn test_types_stream_access_point_serde() {
        let ret = StreamAccessPoint::from_str("0");
        assert!(ret.is_err());

        let value = "1";
        let sap = StreamAccessPoint::from_str(&value).unwrap();
        assert_eq!(sap, StreamAccessPoint::Type1);

        let ser = sap.to_string();
        assert_eq!(value, ser.as_str());
    }

    #[test]
    fn test_types_fourcc() {
        let value = "TSET MJPG H264 VP80";
        let list = ListOfFourCC::from_str(&value).unwrap();

        assert_eq!(
            list.value,
            vec![
                FourCC::from(0x54534554),
                FourCC::from(0x4D4A5047),
                FourCC::from(0x48323634),
                FourCC::from(0x56503830)
            ]
        );

        let ser = list.to_string();

        assert_eq!(value, ser.as_str());
    }

    #[test]
    fn test_types_string_vector() {
        let value = "Hello World !";
        let list = StringVector::from_str(&value).unwrap();

        assert_eq!(list.value, vec!["Hello", "World", "!"]);

        let ser = list.to_string();

        assert_eq!(value, ser.as_str());
    }

    #[test]
    fn test_types_list_of_profiles_serde() {
        let value = "urn:mpeg:dash:profile:isoff-live:2011,urn:mpeg:dash:profile:cmaf:2019,https://example.com/profile/test";
        let list = ListOfProfiles::from_str(&value).unwrap();

        assert_eq!(
            list.value,
            vec![
                Profile::IsoLive,
                Profile::Cmaf,
                Profile::Other("https://example.com/profile/test".to_string())
            ]
        );

        let ser = list.to_string();

        assert_eq!(value, ser.as_str());
    }

    #[test]
    fn test_types_single_range_type_serde_full() {
        let plain = "100-200";
        let range = SingleRFC7233Range::from_str(&plain).unwrap();

        assert_eq!(
            range,
            SingleRFC7233Range {
                start: Some(100),
                end: Some(200)
            }
        );

        let ser = range.to_string();

        assert_eq!(plain, ser.as_str());
    }

    #[test]
    fn test_types_single_range_type_serde_start_only() {
        let plain = "100-";
        let range = SingleRFC7233Range::from_str(&plain).unwrap();

        assert_eq!(
            range,
            SingleRFC7233Range {
                start: Some(100),
                end: None
            }
        );

        let ser = range.to_string();

        assert_eq!(plain, ser.as_str());
    }

    #[test]
    fn test_types_single_range_type_serde_end_only() {
        let plain = "-200";
        let range = SingleRFC7233Range::from_str(&plain).unwrap();

        assert_eq!(
            range,
            SingleRFC7233Range {
                start: None,
                end: Some(200)
            }
        );

        let ser = range.to_string();

        assert_eq!(plain, ser.as_str());
    }

    #[test]
    fn test_types_single_range_type_serde_empty() {
        let plain = "";
        let range = SingleRFC7233Range::from_str(&plain).unwrap();

        assert_eq!(
            range,
            SingleRFC7233Range {
                start: None,
                end: None
            }
        );

        let ser = range.to_string();

        assert_eq!(plain, ser.as_str());
    }

    #[test]
    fn test_types_single_range_type_invalid_format() {
        let plain = "abc-xyz";
        let range = SingleRFC7233Range::from_str(&plain);

        assert!(range.is_err());
    }

    #[test]
    fn test_types_url_type_serde() {
        let xml = r#"<Url sourceURL="http://example.com/video.mp4" range="100-200"/>"#;

        let ret = quick_xml::de::from_str::<Url>(&xml).unwrap();

        assert_eq!(
            ret,
            Url {
                source_url: Some("http://example.com/video.mp4".to_string()),
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
