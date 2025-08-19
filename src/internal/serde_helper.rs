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
        String::deserialize(deserializer)?.parse().map_err(serde::de::Error::custom)
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
        let s: String = String::deserialize(deserializer)?;
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
        let opt_s: Option<String> = Option::deserialize(deserializer)?;
        let Some(s) = opt_s else { return Ok(None) };
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
