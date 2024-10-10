use std::{fmt::Display, result, str::FromStr};

use serde::{Deserialize, Deserializer, Serializer};

pub fn serialize_of_list_type<S, T, I>(value: &T, serializer: S) -> result::Result<S::Ok, S::Error>
where
    S: Serializer,
    for<'a> &'a T: IntoIterator<Item = &'a I>,
    I: Display,
{
    let joined = value
        .into_iter()
        .map(|item| item.to_string())
        .collect::<Vec<String>>()
        .join(" ");
    serializer.serialize_str(&joined)
}

pub fn deserialize_of_list_type<'de, D, T, I>(deserializer: D) -> result::Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: FromIterator<I>,
    I: FromStr,
    I::Err: Display,
{
    let s = String::deserialize(deserializer)?;
    let items = s
        .split_whitespace()
        .map(|item| I::from_str(item).map_err(serde::de::Error::custom))
        .collect::<result::Result<Vec<I>, D::Error>>()?;
    Ok(T::from_iter(items))
}
