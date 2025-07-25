use std::fmt;
use std::time::Duration;

use serde::de::{self, Deserializer, Error, MapAccess, SeqAccess, Visitor};
use serde::{Deserialize, Serializer};

pub fn number_to_string<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let value = Deserialize::deserialize(deserializer)?;
    match value {
        serde_json::Value::Number(num) => Ok(num.to_string()),
        serde_json::Value::String(s) => Ok(s),
        _ => Err(D::Error::custom("expected number or string")),
    }
}

pub fn opt_string_to_u32<'de, D>(
    deserializer: D,
) -> Result<Option<u32>, D::Error>
where
    D: Deserializer<'de>,
{
    let value = Deserialize::deserialize(deserializer)?;
    match value {
        serde_json::Value::Number(num) => {
            Ok(Some(num.as_u64().unwrap() as u32))
        }
        serde_json::Value::String(s) => {
            s.parse::<u32>().map_err(de::Error::custom).map(Some)
        }
        _ => Ok(None),
    }
}

pub fn opt_number_to_string<'de, D>(
    deserializer: D,
) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let value = Deserialize::deserialize(deserializer)?;
    match value {
        serde_json::Value::Number(num) => Ok(Some(num.to_string())),
        serde_json::Value::String(s) => Ok(Some(s)),
        _ => Ok(None),
    }
}

pub fn deserialize_maybe_vec<'de, D, T>(
    deserializer: D,
) -> Result<Vec<T>, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de>,
{
    struct MaybeVecVisitor<T>(std::marker::PhantomData<T>);

    impl<'de, T> Visitor<'de> for MaybeVecVisitor<T>
    where
        T: Deserialize<'de>,
    {
        type Value = Vec<T>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("null, an object, or a sequence of objects")
        }

        fn visit_unit<E>(self) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(Vec::new())
        }

        fn visit_none<E>(self) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(Vec::new())
        }

        fn visit_seq<A>(self, seq: A) -> Result<Self::Value, A::Error>
        where
            A: SeqAccess<'de>,
        {
            Deserialize::deserialize(de::value::SeqAccessDeserializer::new(seq))
        }

        fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
        where
            A: MapAccess<'de>,
        {
            Ok(vec![Deserialize::deserialize(
                de::value::MapAccessDeserializer::new(map),
            )?])
        }
    }

    deserializer.deserialize_any(MaybeVecVisitor(std::marker::PhantomData))
}

pub fn duration_from_millis<'de, D>(
    deserializer: D,
) -> Result<Duration, D::Error>
where
    D: Deserializer<'de>,
{
    let millis = u64::deserialize(deserializer)?;
    Ok(Duration::from_millis(millis))
}

pub fn opt_duration_from_millis<'de, D>(
    deserializer: D,
) -> Result<Option<Duration>, D::Error>
where
    D: Deserializer<'de>,
{
    let millis = Option::<u64>::deserialize(deserializer)?;
    Ok(millis.map(Duration::from_millis))
}

pub fn duration_to_secs<S>(
    duration: &Duration,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_u64(duration.as_secs())
}
