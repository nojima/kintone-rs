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
        String::deserialize(deserializer)?
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
