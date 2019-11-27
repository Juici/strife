//! Models the [snowflake] format used by Discord for uniquely identifiable
//! descriptors (IDs).
//!
//! These IDs are guaranteed to be unique across all of Discord, except in some
//! unique scenarios in which child objects share their parent's ID.
//!
//! [snowflake]: https://discordapp.com/developers/docs/reference#snowflakes

use std::fmt::{self, Display};

use chrono::{DateTime, TimeZone, Utc};
use serde::{
    de::{self, Deserialize, Deserializer},
    ser::{Serialize, Serializer},
};

/// Discord epoch is the first second of 2015.
const DISCORD_EPOCH: u64 = 1_420_070_400_000;

/// A [`Snowflake`] is a 64 bit unique ID.
///
/// [`Snowflake`]: https://discordapp.com/developers/docs/reference#snowflakes
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Snowflake(u64);

impl Snowflake {
    /// Gets the timestamp that the snowflake was created at.
    pub fn created_at(&self) -> DateTime<Utc> {
        // Snowflake timestamp is offset.
        let timestamp = (self.0 >> 22) + DISCORD_EPOCH;

        let secs = timestamp / 1000;
        let millis = (timestamp % 1000) * 1_000_000;

        Utc.timestamp(secs as i64, millis as u32)
    }

    /// Immutably borrow the inner ID.
    pub fn as_u64(&self) -> &u64 {
        &self.0
    }

    /// Mutably borrow the inner ID.
    pub fn as_u64_mut(&mut self) -> &mut u64 {
        &mut self.0
    }
}

impl Display for Snowflake {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl PartialEq<u64> for Snowflake {
    fn eq(&self, other: &u64) -> bool {
        self.0 == *other
    }
}

impl PartialEq<Snowflake> for u64 {
    fn eq(&self, other: &Snowflake) -> bool {
        *self == other.0
    }
}

impl From<u64> for Snowflake {
    fn from(num: u64) -> Self {
        Snowflake(num)
    }
}

impl From<Snowflake> for u64 {
    fn from(id: Snowflake) -> Self {
        id.0 as u64
    }
}

impl Serialize for Snowflake {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u64(self.0)
    }
}

impl<'de> Deserialize<'de> for Snowflake {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct Visitor;

        impl<'de> de::Visitor<'de> for Visitor {
            type Value = u64;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("a 64 bit snowflake id")
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(v as u64)
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(v)
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                v.parse::<u64>().map_err(de::Error::custom)
            }
        }

        deserializer.deserialize_any(Visitor).map(Snowflake)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const ID: u64 = 80351110224678912;
    const ID_STR: &str = "80351110224678912";

    const ID_2: u64 = 41771983423143937;

    #[test]
    fn test_serialize() {
        let snowflake = Snowflake(ID);
        let serialized = serde_json::to_string(&snowflake).unwrap();
        assert_eq!(ID_STR, serialized);
    }

    #[test]
    fn test_deserialize() {
        let deserialized: Snowflake = serde_json::from_str(ID_STR).unwrap();
        assert_eq!(Snowflake(ID), deserialized);
    }

    #[test]
    fn test_equal() {
        let snowflake = Snowflake(ID);
        assert_eq!(ID, snowflake);
        assert_eq!(snowflake, ID);
    }

    #[test]
    fn test_not_equal() {
        let snowflake = Snowflake(ID);
        assert_ne!(ID_2, snowflake);
        assert_ne!(snowflake, ID_2);
    }
}