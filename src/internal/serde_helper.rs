#[derive(serde::Deserialize)]
#[serde(untagged)]
enum NumberOrString {
    String(String),
    Number(serde_json::Number),
}

impl std::fmt::Display for NumberOrString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::String(s) => f.write_str(s),
            Self::Number(n) => n.fmt(f),
        }
    }
}

pub(crate) mod stringified {
    use std::fmt::Display;
    use std::str::FromStr;

    use serde::Deserialize;

    // cf. https://stackoverflow.com/questions/75527167/serde-deserialize-string-into-u64
    pub fn deserialize<'de, T, D>(deserializer: D) -> Result<T, D::Error>
    where
        D: serde::Deserializer<'de>,
        T: FromStr,
        <T as FromStr>::Err: Display,
    {
        super::NumberOrString::deserialize(deserializer)?
            .to_string()
            .parse()
            .map_err(serde::de::Error::custom)
    }

    pub fn serialize<T, S>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
    where
        T: Display,
        S: serde::Serializer,
    {
        serializer.serialize_str(&value.to_string())
    }
}

pub(crate) mod stringified_or_empty {
    use std::fmt::Display;
    use std::str::FromStr;

    use serde::Deserialize;

    // cf. https://stackoverflow.com/questions/75527167/serde-deserialize-string-into-u64
    pub fn deserialize<'de, T, D>(deserializer: D) -> Result<Option<T>, D::Error>
    where
        D: serde::Deserializer<'de>,
        T: FromStr,
        <T as FromStr>::Err: Display,
    {
        let s = super::NumberOrString::deserialize(deserializer)?.to_string();
        if s.is_empty() {
            return Ok(None);
        }
        let v = s.parse::<T>().map_err(serde::de::Error::custom)?;
        Ok(Some(v))
    }

    pub fn serialize<T, S>(value: &Option<T>, serializer: S) -> Result<S::Ok, S::Error>
    where
        T: Display,
        S: serde::Serializer,
    {
        match value {
            Some(v) => serializer.serialize_str(&v.to_string()),
            None => serializer.serialize_str(""),
        }
    }
}

pub(crate) mod option_stringified {
    use std::fmt::Display;
    use std::str::FromStr;

    use serde::Deserialize;

    // cf. https://stackoverflow.com/questions/75527167/serde-deserialize-string-into-u64
    pub fn deserialize<'de, T, D>(deserializer: D) -> Result<Option<T>, D::Error>
    where
        D: serde::Deserializer<'de>,
        T: FromStr,
        <T as FromStr>::Err: Display,
    {
        let opt_s: Option<super::NumberOrString> = Option::deserialize(deserializer)?;
        let Some(s) = opt_s else { return Ok(None) };
        let s = s.to_string();
        if s.is_empty() {
            return Ok(None);
        }
        let v = s.parse::<T>().map_err(serde::de::Error::custom)?;
        Ok(Some(v))
    }

    pub fn serialize<T, S>(v: &Option<T>, serializer: S) -> Result<S::Ok, S::Error>
    where
        T: Display,
        S: serde::Serializer,
    {
        match v {
            Some(value) => serializer.serialize_str(&value.to_string()),
            None => serializer.serialize_none(),
        }
    }
}

/// Some field defaults use the empty string instead of null.
pub(crate) fn empty_string_as_none<'de, T, D>(deserializer: D) -> Result<Option<T>, D::Error>
where
    T: serde::de::DeserializeOwned,
    D: serde::Deserializer<'de>,
{
    let value = <serde_json::Value as serde::Deserialize>::deserialize(deserializer)?;
    if value.is_null() || value.as_str() == Some("") {
        return Ok(None);
    }
    serde_json::from_value(value).map(Some).map_err(serde::de::Error::custom)
}

pub(crate) fn bool_or_string<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: serde::Deserializer<'de>,
{
    #[derive(serde::Deserialize)]
    #[serde(untagged)]
    enum BoolOrString {
        Bool(bool),
        String(String),
    }
    match <BoolOrString as serde::Deserialize>::deserialize(deserializer)? {
        BoolOrString::Bool(value) => Ok(value),
        BoolOrString::String(value) => value.parse().map_err(serde::de::Error::custom),
    }
}

pub(crate) fn empty_string_as_vec<'de, T, D>(deserializer: D) -> Result<Vec<T>, D::Error>
where
    T: serde::de::DeserializeOwned,
    D: serde::Deserializer<'de>,
{
    let value = <serde_json::Value as serde::Deserialize>::deserialize(deserializer)?;
    if value.as_str() == Some("") {
        return Ok(Vec::new());
    }
    serde_json::from_value(value).map_err(serde::de::Error::custom)
}

pub(crate) fn optional_field_datetime<'de, D>(
    deserializer: D,
) -> Result<Option<chrono::DateTime<chrono::FixedOffset>>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let value = <Option<String> as serde::Deserialize>::deserialize(deserializer)?;
    let Some(mut value) = value.filter(|s| !s.is_empty()) else {
        return Ok(None);
    };
    // The field API also returns minute precision timestamps (e.g. 2012-07-19T00:00Z).
    if value.len() == 17 && value.ends_with('Z') {
        value.insert_str(16, ":00");
    }
    chrono::DateTime::parse_from_rfc3339(&value)
        .map(Some)
        .map_err(serde::de::Error::custom)
}

pub(crate) mod optional_field_time {
    use chrono::NaiveTime;
    use serde::{Deserialize, Serializer};

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<NaiveTime>, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = Option::<String>::deserialize(deserializer)?;
        let Some(value) = value.filter(|s| !s.is_empty()) else {
            return Ok(None);
        };
        NaiveTime::parse_from_str(&value, "%H:%M")
            .or_else(|_| NaiveTime::parse_from_str(&value, "%H:%M:%S"))
            .map(Some)
            .map_err(serde::de::Error::custom)
    }

    pub fn serialize<S: Serializer>(
        value: &Option<NaiveTime>,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        match value {
            Some(value) => serializer.serialize_str(&value.format("%H:%M").to_string()),
            None => serializer.serialize_none(),
        }
    }
}
