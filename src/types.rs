use std::{fmt, ops::Deref, str::FromStr};

use chrono::{DateTime, Local, NaiveDateTime, Utc};
use num::{integer::gcd, rational, BigInt};
use serde::{Deserialize, Serialize};
use serde_with::{DeserializeFromStr, SerializeDisplay};

use crate::{definition::Profile, entity::*, error::MpdError, Result};

/// xlink:acture
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub enum XLinkActure {
    OnLoad,
    #[default]
    OnRequest,
}

/// xs:anyURI
#[derive(Debug, Default, Clone, SerializeDisplay, DeserializeFromStr, PartialEq, Eq, Hash)]
pub struct XsAnyURI {
    value: String,
}

impl fmt::Display for XsAnyURI {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl FromStr for XsAnyURI {
    type Err = MpdError;

    fn from_str(s: &str) -> Result<Self> {
        Ok(Self {
            value: s.to_string(),
        })
    }
}

impl<T> From<T> for XsAnyURI
where
    T: AsRef<str>,
{
    fn from(value: T) -> Self {
        Self {
            value: value.as_ref().to_string(),
        }
    }
}

/// xs:integer
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

    fn from_str(s: &str) -> Result<Self> {
        if !PATTERN_INTEGER.is_match(s) {
            return Err(MpdError::UnmatchedPattern);
        }

        let value = s
            .parse::<BigInt>()
            .map_err(|_| MpdError::InvalidData("Failed to parse xs:integer"))?;

        Ok(Self { value })
    }
}

/// xs:ID
///
/// <b>※Warn</b> : No check is made for uniqueness within an XML instance.
#[derive(Debug, Default, Clone, SerializeDisplay, DeserializeFromStr, PartialEq, Eq, Hash)]
pub struct XsId {
    value: String,
}

impl fmt::Display for XsId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl FromStr for XsId {
    type Err = MpdError;

    fn from_str(s: &str) -> Result<Self> {
        // xs:token
        let value = s.trim().replace("\n", " ").replace("\r\n", " ");

        if !PATTERN_NC_NAME.is_match(&value) {
            return Err(MpdError::UnmatchedPattern);
        }

        Ok(Self { value })
    }
}

/// xs:language
#[derive(Debug, Default, Clone, SerializeDisplay, DeserializeFromStr, PartialEq, Eq, Hash)]
pub struct XsLanguage {
    value: String,
}

impl fmt::Display for XsLanguage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl FromStr for XsLanguage {
    type Err = MpdError;

    fn from_str(s: &str) -> Result<Self> {
        // xs:token
        let value = s.trim().replace("\n", " ").replace("\r\n", " ");

        if !PATTERN_LANG.is_match(&value) {
            return Err(MpdError::UnmatchedPattern);
        }

        Ok(Self { value })
    }
}

/// xs:dateTime
#[derive(Debug, Default, Clone, SerializeDisplay, DeserializeFromStr, PartialEq, Eq, Hash)]
pub struct XsDateTime {
    value: DateTime<Utc>,
}

impl fmt::Display for XsDateTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            self.value
                .to_rfc3339_opts(chrono::SecondsFormat::AutoSi, true) // 小数点以下の扱いをAutoにしているがこれで問題ないか
        )
    }
}

impl FromStr for XsDateTime {
    type Err = MpdError;

    fn from_str(s: &str) -> Result<Self> {
        let time_part = s.split('T').nth(1).ok_or(MpdError::UnmatchedPattern)?;

        let value = if time_part.contains('Z') || time_part.contains('+') || time_part.contains('-')
        {
            DateTime::parse_from_rfc3339(s)?.to_utc()
        } else {
            let datetime = NaiveDateTime::parse_from_str(s, "%Y-%m-%dT%H:%M:%S%.f")?
                .and_local_timezone(Local)
                .unwrap();
            datetime.to_utc()
        };

        Ok(Self { value })
    }
}

impl From<DateTime<Utc>> for XsDateTime {
    fn from(value: DateTime<Utc>) -> Self {
        Self { value }
    }
}

/// xs:duration
#[derive(Debug, Default, Clone, SerializeDisplay, DeserializeFromStr, PartialEq, Eq)]
pub struct XsDuration {
    value: std::time::Duration,
    is_negative: bool,
}

impl Deref for XsDuration {
    type Target = std::time::Duration;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl fmt::Display for XsDuration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut output = if self.is_negative {
            String::from("-PT")
        } else {
            String::from("PT")
        };

        let mut seconds = self.value.as_secs();
        let mut nanos = self.value.subsec_nanos();

        let hours = seconds / 3600;
        seconds = seconds % 3600;

        if hours != 0 {
            output.push_str(&format!("{hours}H"));
        }

        let minutes = seconds / 60;
        seconds = seconds % 60;

        if minutes != 0 {
            output.push_str(&format!("{minutes}M"));
        }

        if nanos != 0 {
            // 末尾の０を削除
            while nanos % 10 == 0 {
                nanos /= 10;
            }

            output.push_str(&format!("{}.{}S", seconds, nanos));
        } else if seconds != 0 {
            output.push_str(&format!("{}S", seconds));
        };

        write!(f, "{output}")
    }
}

impl FromStr for XsDuration {
    type Err = MpdError;

    fn from_str(s: &str) -> Result<Self> {
        let mut chars = s.chars().peekable();

        // Check for negative
        let is_negative = if chars.peek() == Some(&'-') {
            chars.next();
            true
        } else {
            false
        };

        if chars.next() != Some('P') {
            return Err(MpdError::UnmatchedPattern);
        }

        let mut duration = std::time::Duration::default();
        let mut flag = 0b0000_0000;

        let mut value = String::new();

        while let Some(c) = chars.next() {
            if c.is_digit(10) || (c == '.' && !value.contains('.')) {
                value.push(c);
            } else {
                if c == 'T' {
                    if chars.peek() != None {
                        flag |= 0b0000_1000;
                        continue;
                    } else {
                        return Err(MpdError::UnmatchedPattern);
                    }
                } else if value.is_empty() {
                    return Err(MpdError::UnmatchedPattern);
                }

                match c {
                    'Y' if flag == 0b0000_0000 => {
                        let years = value.parse::<u64>()? * 365 * 24 * 60 * 60;
                        duration += std::time::Duration::from_secs(years);
                        flag |= 0b0000_0001;
                    }
                    'M' if flag < 0b0000_0010 => {
                        let months = value.parse::<u64>()? * 30 * 24 * 60 * 60;
                        duration += std::time::Duration::from_secs(months);
                        flag |= 0b0000_0010;
                    }
                    'D' if flag < 0b0000_0100 => {
                        let days = value.parse::<u64>()? * 24 * 60 * 60;
                        duration += std::time::Duration::from_secs(days);
                        flag |= 0b0000_0100;
                    }
                    'H' if flag >= 0b0000_1000 && flag < 0b0001_0000 => {
                        let hours = value.parse::<u64>()? * 60 * 60;
                        duration += std::time::Duration::from_secs(hours);
                        flag |= 0b0001_0000;
                    }
                    'M' if flag >= 0b0000_1000 && flag < 0b0010_0000 => {
                        let minutes = value.parse::<u64>()? * 60;
                        duration += std::time::Duration::from_secs(minutes);
                        flag |= 0b0010_0000;
                    }
                    'S' if flag >= 0b0000_1000 && flag < 0b0100_0000 => {
                        duration += if value.contains('.') {
                            let nanos = (value.parse::<f64>()? * 1_000_000_000.0) as u64;
                            std::time::Duration::from_nanos(nanos)
                        } else {
                            std::time::Duration::from_secs(value.parse::<u64>()?)
                        };
                    }
                    _ => return Err(MpdError::UnmatchedPattern),
                }

                value.clear();
            }
        }

        if flag & 0b1111_0111 != 0 {
            Ok(Self {
                value: duration,
                is_negative,
            })
        } else {
            Err(MpdError::UnmatchedPattern)
        }
    }
}

impl From<std::time::Duration> for XsDuration {
    fn from(value: std::time::Duration) -> Self {
        Self {
            value,
            is_negative: false,
        }
    }
}

/// 4CC as per latest 14496-12
#[derive(Default, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct FourCC {
    value: [u8; 4],
}

impl Deref for FourCC {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        std::str::from_utf8(&self.value).unwrap()
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

/// Ratio Type for sar and par
#[derive(Debug, Default, Clone, SerializeDisplay, DeserializeFromStr, PartialEq, Eq, Hash)]
pub struct Ratio {
    horizontal: u32,
    vertical: u32,
}

impl fmt::Display for Ratio {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.horizontal, self.vertical)
    }
}

impl FromStr for Ratio {
    type Err = MpdError;

    fn from_str(s: &str) -> Result<Self> {
        let parts: Vec<&str> = s.split(':').collect();

        if parts.len() == 2 {
            let horizontal = parts[0].parse::<u32>()?;
            let vertical = parts[1].parse::<u32>()?;

            Ok(Self {
                horizontal,
                vertical,
            })
        } else {
            Err(MpdError::UnmatchedPattern)
        }
    }
}

impl From<(u32, u32)> for Ratio {
    fn from(value: (u32, u32)) -> Self {
        let (numer, denom) = value;
        let divisor = gcd(numer, denom);
        Self {
            horizontal: numer / divisor,
            vertical: denom / divisor,
        }
    }
}

/// Type for Frame Rate
#[derive(Debug, Default, Clone, SerializeDisplay, DeserializeFromStr, PartialEq, Eq, Hash)]
pub struct FrameRate {
    value: rational::Ratio<u32>,
}

impl fmt::Display for FrameRate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}/{}", self.value.numer(), self.value.denom())
    }
}

impl FromStr for FrameRate {
    type Err = MpdError;

    fn from_str(s: &str) -> Result<Self> {
        let parts: Vec<&str> = s.split('/').collect();

        if parts.len() == 0 || parts.len() > 2 {
            return Err(MpdError::UnmatchedPattern);
        }

        let numer = parts[0].parse::<u32>()?;

        let denom = if parts.len() == 2 {
            parts[1].parse::<u32>()?
        } else {
            1
        };

        if denom == 0 {
            return Err(MpdError::InvalidData(
                "Don't set ZERO to the framerate denom",
            ));
        }

        Ok(Self {
            value: rational::Ratio::new(numer, denom),
        })
    }
}

/// String without white spaces
///
/// base : xs:string
#[derive(Debug, Default, Clone, SerializeDisplay, DeserializeFromStr, PartialEq, Eq, Hash)]
pub struct NoWhitespace {
    value: String,
}

impl fmt::Display for NoWhitespace {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl FromStr for NoWhitespace {
    type Err = MpdError;

    fn from_str(s: &str) -> Result<Self> {
        if !PATTERN_NO_WHITESPACE.is_match(s) {
            return Err(MpdError::UnmatchedPattern);
        }

        Ok(Self {
            value: s.to_string(),
        })
    }
}

/// Single RFC7233 Byte Range
///
/// RFC7233
#[derive(Debug, Default, Clone, SerializeDisplay, DeserializeFromStr, PartialEq, Eq, Hash)]
pub struct SingleByteRange {
    first: u32,
    last: Option<u32>,
}

impl fmt::Display for SingleByteRange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let first = self.first.to_string();
        let last = self.last.map_or("".to_string(), |n| n.to_string());
        write!(f, "{}-{}", first, last)
    }
}

impl FromStr for SingleByteRange {
    type Err = MpdError;

    fn from_str(s: &str) -> Result<Self> {
        let parts: Vec<&str> = s.split('-').collect();

        if parts.len() != 2 {
            return Err(MpdError::UnmatchedPattern);
        }

        let first = parts[0].parse::<u32>()?;
        let last = if !parts[1].is_empty() {
            Some(parts[1].parse::<u32>()?)
        } else {
            None
        };

        if last.is_some_and(|last| last < first) {
            return Err(MpdError::UnmatchedPattern);
        }

        Ok(Self { first, last })
    }
}

impl From<u32> for SingleByteRange {
    fn from(value: u32) -> Self {
        Self {
            first: value,
            last: None,
        }
    }
}

impl TryFrom<(u32, u32)> for SingleByteRange {
    type Error = MpdError;

    fn try_from(value: (u32, u32)) -> Result<Self> {
        if value.0 > value.1 {
            return Err(MpdError::UnmatchedPattern);
        }

        Ok(Self {
            first: value.0,
            last: Some(value.1),
        })
    }
}

/// Type for RFC6838 Content Type
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum ContentType {
    Text,
    Image,
    Audio,
    #[default]
    Video,
    Application,
    Font,
}

/// Stream Access Point type enumeration
#[repr(u8)]
#[derive(Debug, Default, Clone, SerializeDisplay, DeserializeFromStr, PartialEq, Eq, Hash)]
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

    fn try_from(value: u8) -> Result<Self> {
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
        write!(f, "{}", self.clone() as u8)
    }
}

impl FromStr for StreamAccessPoint {
    type Err = MpdError;

    fn from_str(s: &str) -> Result<Self> {
        Ok(StreamAccessPoint::try_from(s.parse::<u8>()?)?)
    }
}

/// RFC6381 fancy-list without enclosing double quotes
///
/// fancy-list := `[charset] "'" [language] "'" id-list`
#[derive(Debug, Default, Clone, SerializeDisplay, DeserializeFromStr, PartialEq, Eq, Hash)]
pub struct FancyList {
    charset: Option<String>,
    language: Option<String>,
    codecs: Vec<String>,
}

impl fmt::Display for FancyList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let charset = self.charset.as_deref().unwrap_or("");

        if let Some(language) = self.language.as_deref() {
            write!(f, "{}'{}'{}", charset, language, self.codecs.join(","))
        } else {
            write!(f, "{}{}", charset, self.codecs.join(","))
        }
    }
}

impl FromStr for FancyList {
    type Err = MpdError;

    fn from_str(s: &str) -> Result<Self> {
        let Some(caps) = PATTERN_FANCY.captures(s) else {
            return Err(MpdError::UnmatchedPattern);
        };

        let charset = caps.name("charset").map(|s| s.as_str().to_string());
        let language = caps.name("language").map(|s| s.as_str().to_string());
        let codecs = caps
            .name("codecs")
            .unwrap()
            .as_str()
            .split(",")
            .map(|s| s.to_string())
            .collect();

        Ok(Self {
            charset,
            language,
            codecs,
        })
    }
}

/// RFC6381 simp-list without enclosing double quotes
///
/// simp-list := `id-simple *( "," id-simple )`
#[derive(Debug, Default, Clone, SerializeDisplay, DeserializeFromStr, PartialEq, Eq, Hash)]
pub struct SimpList {
    codecs: Vec<String>,
}

impl fmt::Display for SimpList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.codecs.join(","))
    }
}

impl FromStr for SimpList {
    type Err = MpdError;

    fn from_str(s: &str) -> Result<Self> {
        if !PATTERN_SIMPLE.is_match(s) {
            return Err(MpdError::UnmatchedPattern);
        }

        let codecs = s.split(",").map(|s| s.to_string()).collect();
        Ok(Self { codecs })
    }
}

/// CodecsType
#[derive(Debug, Clone, SerializeDisplay, DeserializeFromStr, PartialEq, Eq, Hash)]
pub enum Codecs {
    Fancy(FancyList),
    Simp(SimpList),
}

impl fmt::Display for Codecs {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Fancy(fancy) => fancy.to_string(),
            Self::Simp(simp) => simp.to_string(),
        };

        write!(f, "{s}")
    }
}

impl FromStr for Codecs {
    type Err = MpdError;

    fn from_str(s: &str) -> Result<Self> {
        if s.contains("'") || s.contains(".") {
            Ok(Self::Fancy(FancyList::from_str(s)?))
        } else {
            Ok(Self::Simp(SimpList::from_str(s)?))
        }
    }
}

/// Tag
///
/// base : xs:string
#[derive(Debug, Default, Clone, SerializeDisplay, DeserializeFromStr, PartialEq, Eq, Hash)]
pub struct Tag {
    value: String,
}

impl fmt::Display for Tag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl FromStr for Tag {
    type Err = MpdError;

    fn from_str(s: &str) -> Result<Self> {
        Ok(Tag {
            value: s.to_string(),
        })
    }
}

impl<S> From<S> for Tag
where
    S: AsRef<str>,
{
    fn from(value: S) -> Self {
        Self {
            value: value.as_ref().to_string(),
        }
    }
}

/// Video Scan type enumeration
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum VideoScan {
    Progressive,
    InterLaced,
    #[default]
    Unknown,
}

/// Event Coding
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum ContentEncoding {
    #[default]
    Base64,
}

/// Switching Type type enumeration
///
/// ref : Table 7
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum SwitchingType {
    #[default]
    Media,
    Bitstream,
}

/// Random Access Type type enumeration
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum RandomAccessType {
    #[default]
    Closed,
    Open,
    Gradual,
}

/// Producer Reference Time type enumeration
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum ProducerReferenceTimeType {
    #[default]
    Encoder,
    Captured,
    Application,
}

/// Rating Source type enumeration
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum Source {
    #[default]
    Content,
    Statistics,
    // need @source_description
    Other,
}

/// Operating Quality parameters applied media type enumeration
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum QualityMediaType {
    Video,
    Audio,
    #[default]
    Any,
}

/// Operating Bandwidth parameters applied media type enumeration
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum BandwidthMediaType {
    Video,
    Audio,
    Any,
    #[default]
    All,
}

/// Preselection Order type enumeration
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "kebab-case")]
pub enum PreselectionOrderType {
    #[default]
    Undefined,
    TimeOrdered,
    FullyOrdered,
}

/// Presentation type enumeration
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum PresentationType {
    #[default]
    Static,
    Dynamic,
}

/// List of Profiles
///
/// base : xs:string
#[derive(Debug, Default, Clone, SerializeDisplay, DeserializeFromStr, PartialEq, Eq, Hash)]
pub struct ListOfProfiles {
    value: Vec<Profile>,
}

impl Deref for ListOfProfiles {
    type Target = Vec<Profile>;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl fmt::Display for ListOfProfiles {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let joined = self
            .value
            .iter()
            .map(|p| p.to_string())
            .collect::<Vec<String>>()
            .join(",");
        write!(f, "{joined}")
    }
}

impl FromStr for ListOfProfiles {
    type Err = MpdError;

    fn from_str(s: &str) -> Result<Self> {
        let value = s
            .split(",")
            .map(|s| Profile::from_str(s))
            .collect::<Result<Vec<Profile>>>()?;
        Ok(Self { value })
    }
}

impl From<Vec<Profile>> for ListOfProfiles {
    fn from(value: Vec<Profile>) -> Self {
        Self { value }
    }
}

impl<T> From<&[T]> for ListOfProfiles
where
    T: Into<Profile> + Clone,
{
    fn from(value: &[T]) -> Self {
        let value = value.iter().map(|t| t.clone().into()).collect();
        Self { value }
    }
}

/// Whitespace separated list
#[derive(Debug, Default, Clone, SerializeDisplay, DeserializeFromStr, PartialEq, Eq, Hash)]
pub struct WhitespaceSeparatedList<T: fmt::Display + FromStr> {
    value: Vec<T>,
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

/// Whitespace separated list of unsigned integers
pub type UIntVector = WhitespaceSeparatedList<u32>;
/// Whitespace separated list of strings
pub type StringVector = WhitespaceSeparatedList<String>;
/// Whitespace separated list of 4CC
pub type ListOfFourCC = WhitespaceSeparatedList<FourCC>;

#[derive(Debug, Default, Clone, SerializeDisplay, DeserializeFromStr, PartialEq, Eq, Hash)]
pub struct AudioSamplingRate(UIntVector);

impl Deref for AudioSamplingRate {
    type Target = UIntVector;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl fmt::Display for AudioSamplingRate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.to_string())
    }
}

impl FromStr for AudioSamplingRate {
    type Err = MpdError;

    fn from_str(s: &str) -> Result<Self> {
        let items = s
            .split_whitespace()
            .map(|s| s.parse::<u32>().map_err(|err| MpdError::ParseIntError(err)))
            .collect::<Result<Vec<u32>>>()?;

        if items.len() > 0 && items.len() < 3 {
            Ok(AudioSamplingRate(WhitespaceSeparatedList::from(items)))
        } else {
            Err(MpdError::InvalidData(
                "The number of Audio sampling rate must be between 1 and 2",
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_types_xs_integer_valid() {
        assert!(XsInteger::from_str("122").is_ok());
        assert!(XsInteger::from_str("00122").is_ok());
        assert!(XsInteger::from_str("0").is_ok());
        assert!(XsInteger::from_str("-3").is_ok());
        assert!(XsInteger::from_str("+3").is_ok());
    }

    #[test]
    fn test_types_xs_integer_invalid() {
        assert!(XsInteger::from_str("3.").is_err());
        assert!(XsInteger::from_str("3.0").is_err());
        assert!(XsInteger::from_str("").is_err());
    }

    #[test]
    fn test_types_xs_id_valid() {
        assert!(XsId::from_str("myElement").is_ok());
        assert!(XsId::from_str("_my.Element").is_ok());
        assert!(XsId::from_str("my-Element").is_ok());

        // xs:token
        assert_eq!(
            XsId::from_str("  _my.Element").unwrap().to_string(),
            "_my.Element".to_string()
        );
    }

    #[test]
    fn test_types_xs_id_invalid() {
        assert!(XsId::from_str("pre:myElement").is_err());
        assert!(XsId::from_str("-myelement").is_err());
        assert!(XsId::from_str("").is_err());
    }

    #[test]
    fn test_types_xs_lang_valid() {
        assert!(XsLanguage::from_str("en").is_ok());
        assert!(XsLanguage::from_str("en-GB").is_ok());
        assert!(XsLanguage::from_str("ja").is_ok());
        assert!(XsLanguage::from_str("i-navajo").is_ok());
        assert!(XsLanguage::from_str("x-Newspeak").is_ok());
        assert!(XsLanguage::from_str("any-value-with-short-partsen").is_ok());
    }

    #[test]
    fn test_types_xs_lang_invalid() {
        assert!(XsLanguage::from_str("longerThan8").is_err());
        assert!(XsLanguage::from_str("").is_err());
    }

    #[test]
    fn test_types_xs_datetime_valid() {
        assert!(XsDateTime::from_str("2004-04-12T13:20:00").is_ok());
        assert!(XsDateTime::from_str("2004-04-12T13:20:15.5").is_ok());
        assert!(XsDateTime::from_str("2004-04-12T13:20:00-05:00").is_ok());
        assert!(XsDateTime::from_str("2004-04-12T13:20:00Z").is_ok());
    }

    #[test]
    fn test_types_xs_datetime_invalid() {
        assert!(XsDateTime::from_str("2004-04-12T13:00").is_err());
        assert!(XsDateTime::from_str("2004-04-1213:20:00").is_err());
        assert!(XsDateTime::from_str("99-04-12T13:00").is_err());
        assert!(XsDateTime::from_str("2004-04-12").is_err());
        assert!(XsDateTime::from_str("").is_err());
    }

    #[test]
    fn test_types_xs_datetime_parse() {
        let datetime = XsDateTime::from_str("2004-04-12T13:20:00-05:00").unwrap();
        assert_eq!(&datetime.to_string(), "2004-04-12T18:20:00Z");

        let datetime = XsDateTime::from_str("2004-04-12T13:20:15.5").unwrap();
        assert_eq!(&datetime.to_string(), "2004-04-12T04:20:15.500Z");
    }

    #[test]
    fn test_types_xs_duration_valid() {
        assert!(XsDuration::from_str("P2Y6M5DT12H35M30S").is_ok());
        assert!(XsDuration::from_str("P1DT2H").is_ok());
        assert!(XsDuration::from_str("P20M").is_ok());
        assert!(XsDuration::from_str("PT20M").is_ok());
        assert!(XsDuration::from_str("P0Y20M0D").is_ok());
        assert!(XsDuration::from_str("P0Y").is_ok());
        assert!(XsDuration::from_str("-P60D").is_ok());
        assert!(XsDuration::from_str("PT1M30.5S").is_ok());
    }

    #[test]
    fn test_types_xs_duration_invalid() {
        assert!(XsDuration::from_str("P-20M").is_err());
        assert!(XsDuration::from_str("P20MT").is_err());
        assert!(XsDuration::from_str("P1YM5D").is_err());
        assert!(XsDuration::from_str("P15.5Y").is_err());
        assert!(XsDuration::from_str("P1D2H").is_err());
        assert!(XsDuration::from_str("1Y2M").is_err());
        assert!(XsDuration::from_str("P2M1Y").is_err());
        assert!(XsDuration::from_str("P").is_err());
        assert!(XsDuration::from_str("PT15.S").is_err());
        assert!(XsDuration::from_str("").is_err());
    }

    #[test]
    fn test_types_xs_duration_parse() {
        let duration = XsDuration::from_str("P2Y6M5DT12H35M30S").unwrap();
        assert_eq!(&duration.to_string(), "PT21972H35M30S");

        let duration = XsDuration::from_str("P20M").unwrap();
        assert_eq!(&duration.to_string(), "PT14400H");

        let duration = XsDuration::from_str("PT20M").unwrap();
        assert_eq!(&duration.to_string(), "PT20M");

        let duration = XsDuration::from_str("PT1M30.5S").unwrap();
        assert_eq!(&duration.to_string(), "PT1M30.5S");

        let duration = XsDuration::from_str("-P2DT1M30.123456789S").unwrap();
        assert_eq!(&duration.to_string(), "-PT48H1M30.123456789S");
    }

    #[test]
    fn test_types_fourcc_valid() {
        assert!(FourCC::from_str("MPEG").is_ok());
    }

    #[test]
    fn test_types_fourcc_invalid() {
        assert!(FourCC::from_str("MPEG2").is_err());
        assert!(FourCC::from_str(" mpeg").is_err());
        assert!(FourCC::from_str("a").is_err());
    }

    #[test]
    fn test_types_ratio_valid() {
        assert!(Ratio::from_str("16:9").is_ok());
        assert!(Ratio::from_str("1920:1080").is_ok());
        assert!(Ratio::from_str("0:1").is_ok());
        assert!(Ratio::from_str("0:0").is_ok());
    }

    #[test]
    fn test_types_ratio_invalid() {
        assert!(Ratio::from_str("16:").is_err());
        assert!(Ratio::from_str(":9").is_err());
        assert!(Ratio::from_str(":").is_err());
        assert!(Ratio::from_str("").is_err());
    }

    #[test]
    fn test_types_ratio_parse() {
        let input = "16:9";
        let ratio = Ratio::from_str(&input).unwrap();
        assert_eq!(&ratio.to_string(), input);

        let input = "0:0";
        let ratio = Ratio::from_str(&input).unwrap();
        assert_eq!(&ratio.to_string(), input);
    }

    #[test]
    fn test_types_framerate_valid() {
        assert!(FrameRate::from_str("30/1").is_ok());
        assert!(FrameRate::from_str("30").is_ok());
        assert!(FrameRate::from_str("30000/1001").is_ok());
        assert!(FrameRate::from_str("0/1").is_ok());
    }

    #[test]
    fn test_types_framerate_invalid() {
        assert!(FrameRate::from_str("30/0").is_err());
        assert!(FrameRate::from_str("29.97/1.0").is_err());
        assert!(FrameRate::from_str("30/").is_err());
        assert!(FrameRate::from_str("/1").is_err());
        assert!(FrameRate::from_str("/").is_err());
        assert!(FrameRate::from_str("").is_err());
    }

    #[test]
    fn test_types_framerate_parse() {
        let input = "30/1";
        let framerate = FrameRate::from_str(&input).unwrap();
        assert_eq!(&framerate.to_string(), input);

        let input = "60";
        let framerate = FrameRate::from_str(&input).unwrap();
        assert_eq!(&framerate.to_string(), "60/1");
    }

    #[test]
    fn test_types_no_whitespace_valid() {
        assert!(NoWhitespace::from_str("HelloWorld").is_ok());
        assert!(NoWhitespace::from_str("1234567890!?/\\@#_,.%$\'\"").is_ok());
    }

    #[test]
    fn test_types_no_whitespace_invalid() {
        assert!(NoWhitespace::from_str("Hello World").is_err());
        assert!(NoWhitespace::from_str("Hello\nWorld").is_err());
        assert!(NoWhitespace::from_str("Hello\r\nWorld").is_err());
        assert!(NoWhitespace::from_str("Hello\tWorld").is_err());
    }

    #[test]
    fn test_types_no_whitespace_parse() {
        let input = "1234567890!?/\\@#_,.%$\'\"";
        let no_whitespace = NoWhitespace::from_str("1234567890!?/\\@#_,.%$\'\"").unwrap();
        assert_eq!(&no_whitespace.to_string(), input);
    }

    #[test]
    fn test_types_single_byte_range_valid() {
        assert!(SingleByteRange::from_str("0-499").is_ok());
        assert!(SingleByteRange::from_str("500-999").is_ok());
        assert!(SingleByteRange::from_str("0-").is_ok());
    }

    #[test]
    fn test_types_single_byte_range_invalid() {
        assert!(SingleByteRange::from_str("-499").is_err());
        assert!(SingleByteRange::from_str("500-499").is_err());
        assert!(SingleByteRange::from_str("0-499,500-999").is_err());
        assert!(SingleByteRange::from_str("0-499,-1").is_err());
        assert!(SingleByteRange::from_str("-").is_err());
        assert!(SingleByteRange::from_str("").is_err());
    }

    #[test]
    fn test_types_single_byte_range_parse() {
        let input = "0-499";
        let range = SingleByteRange::from_str(&input).unwrap();
        assert_eq!(&range.to_string(), input);

        let input = "500-";
        let range = SingleByteRange::from_str(&input).unwrap();
        assert_eq!(&range.to_string(), input);
    }

    #[test]
    fn test_types_fancy_list_valid() {
        assert!(FancyList::from_str("UTF-8'en'avc1.42E01E,mp4a.40.2").is_ok());
        assert!(FancyList::from_str("UTF-8'en'avc1.42E01E, mp4a.40.2").is_ok());
        assert!(FancyList::from_str("avc1.42E01E").is_ok());
        assert!(FancyList::from_str("avc1").is_ok());
    }

    #[test]
    fn test_types_fancy_list_invalid() {
        assert!(FancyList::from_str("UTF-8'en'").is_err());
        assert!(FancyList::from_str("'en'avc1.42E01E,mp4a.40.2").is_err());
        assert!(FancyList::from_str("avc1.42E01E;mp4a.40.2").is_err());
        assert!(FancyList::from_str("avc1.42E01E,").is_err());
        assert!(FancyList::from_str("UTF-8\"en\"avc1.42E01E").is_err());
        assert!(FancyList::from_str("").is_err());
    }

    #[test]
    fn test_types_fancy_list_parse() {
        let input = "UTF-8'en'avc1.42E01E,mp4a.40.2";
        let fancy_list = FancyList::from_str(&input).unwrap();
        assert_eq!(&fancy_list.to_string(), input);

        let input = "avc1.42E01E,mp4a.40.2";
        let fancy_list = FancyList::from_str(&input).unwrap();
        assert_eq!(&fancy_list.to_string(), input);
    }

    #[test]
    fn test_types_simp_list_valid() {
        assert!(SimpList::from_str("avc1").is_ok());
        assert!(SimpList::from_str("avc1,mp4a").is_ok());
        assert!(SimpList::from_str("avc1, mp4a").is_ok());
    }

    #[test]
    fn test_types_simp_list_invalid() {
        assert!(SimpList::from_str("avc1.42E01E").is_err());
        assert!(SimpList::from_str("avc1;mp4a").is_err());
        assert!(SimpList::from_str("avc1,").is_err());
        assert!(SimpList::from_str("").is_err());
    }

    #[test]
    fn test_types_simp_list_parse() {
        let input = "avc1";
        let simp_list = SimpList::from_str(&input).unwrap();
        assert_eq!(&simp_list.to_string(), input);

        let input = "avc1,mp4a";
        let simp_list = SimpList::from_str(&input).unwrap();
        assert_eq!(&simp_list.to_string(), input);
    }

    #[test]
    fn test_types_codecs_parse() {
        let input = "UTF-8'en'avc1.42E01E,mp4a.40.2";
        let codecs = Codecs::from_str(&input).unwrap();
        assert_eq!(codecs, Codecs::Fancy(FancyList::from_str(&input).unwrap()));
        assert_eq!(&codecs.to_string(), input);

        let input = "avc1.42E01E,mp4a";
        let codecs = Codecs::from_str(&input).unwrap();
        assert_eq!(codecs, Codecs::Fancy(FancyList::from_str(&input).unwrap()));
        assert_eq!(&codecs.to_string(), input);

        let input = "avc1,mp4a";
        let codecs = Codecs::from_str(&input).unwrap();
        assert_eq!(codecs, Codecs::Simp(SimpList::from_str(&input).unwrap()));
        assert_eq!(&codecs.to_string(), input);
    }

    #[test]
    fn test_types_list_of_profiles_valid() {
        assert!(ListOfProfiles::from_str("urn:example:resource").is_ok());
        assert!(ListOfProfiles::from_str("https://example.com").is_ok());
        assert!(ListOfProfiles::from_str("urn:example:resource,https://example.com").is_ok());
        assert!(ListOfProfiles::from_str(
            "urn:example:resource,urn:another:resource,https://example.org"
        )
        .is_ok());
        assert!(ListOfProfiles::from_str(
            "urn:example:namespace,https://example.com,http://example.org"
        )
        .is_ok());
    }

    #[test]
    fn test_types_list_of_profiles_invalid() {
        assert!(ListOfProfiles::from_str("urn::invalid:urn").is_err());
        assert!(ListOfProfiles::from_str("urn:missing:section:,http://example.com").is_err());
        assert!(ListOfProfiles::from_str("urn:example:resource,\nhttps://example.com").is_err());
        assert!(ListOfProfiles::from_str("https:/invalid-url").is_err());
        assert!(ListOfProfiles::from_str("ftp://not-supported-url").is_err());
        assert!(ListOfProfiles::from_str("urn:valid, ,https://valid-url.com").is_err());
        assert!(ListOfProfiles::from_str("urn:valid,,https://valid-url.com").is_err());
        assert!(ListOfProfiles::from_str(",").is_err());
        assert!(ListOfProfiles::from_str("").is_err());
    }

    #[test]
    fn test_types_list_of_profiles_parse() {
        let profiles = ListOfProfiles::from_str(
            "urn:example:namespace,https://example.com,http://example.org",
        )
        .unwrap();

        assert_eq!(
            profiles.value,
            [
                Profile::from_str("urn:example:namespace").unwrap(),
                Profile::from_str("https://example.com").unwrap(),
                Profile::from_str("http://example.org").unwrap(),
            ]
        );

        assert_eq!(
            &profiles.to_string(),
            "urn:example:namespace,https://example.com,http://example.org"
        );
    }

    #[test]
    fn test_types_whitespace_separated_list_valid() {
        assert!(UIntVector::from_str("1 2 3 4 5").is_ok());
        assert!(StringVector::from_str("a b c d e").is_ok());
        assert!(ListOfFourCC::from_str("MPEG JPEG H264").is_ok());
        assert!(AudioSamplingRate::from_str("1 2").is_ok());
        assert!(UIntVector::from_str("").is_ok());
    }

    #[test]
    fn test_types_whitespace_separated_list_invalid() {
        assert!(UIntVector::from_str("a b c").is_err());
        assert!(ListOfFourCC::from_str("a,b,c,d,e").is_err());
        assert!(AudioSamplingRate::from_str("1 2 3").is_err());
        assert!(AudioSamplingRate::from_str("").is_err());
    }
}
