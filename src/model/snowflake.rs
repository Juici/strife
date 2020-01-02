//! Models the [snowflake] format used by Discord for uniquely identifiable
//! descriptors (IDs).
//!
//! These IDs are guaranteed to be unique across all of Discord, except in some
//! unique scenarios in which child objects share their parent's ID.
//!
//! [snowflake]: https://discordapp.com/developers/docs/reference#snowflakes

use std::fmt::{self, Display};
use std::mem::MaybeUninit;

use chrono::{DateTime, FixedOffset, TimeZone};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::model::utils::U64Visitor;

/// Discord epoch is the first second of 2015.
const DISCORD_EPOCH: u64 = 1_420_070_400_000;

/// A [`Snowflake`] is a 64 bit unique ID.
///
/// [`Snowflake`]: https://discordapp.com/developers/docs/reference#snowflakes
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Snowflake(u64);

impl Snowflake {
    /// Gets the timestamp that the snowflake was created at.
    pub fn created_at(&self) -> DateTime<FixedOffset> {
        // Snowflake timestamp is offset.
        let timestamp = (self.0 >> 22) + DISCORD_EPOCH;

        let secs = timestamp / 1000;
        let millis = (timestamp % 1000) * 1_000_000;

        FixedOffset::east(0).timestamp(secs as i64, millis as u32)
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

impl AsRef<u64> for Snowflake {
    fn as_ref(&self) -> &u64 {
        &self.0
    }
}

impl AsMut<u64> for Snowflake {
    fn as_mut(&mut self) -> &mut u64 {
        &mut self.0
    }
}

impl Serialize for Snowflake {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Adapted from core::fmt::num impl_Display macro.
        // https://doc.rust-lang.org/src/core/fmt/num.rs.html#192-238

        use crate::internal::DEC_DIGITS_LUT;

        let mut n = self.0;

        // Buffer of 20 is large enough to hold the max u64 value.
        let mut buf = [MaybeUninit::<u8>::uninit(); 20];
        let mut curr = buf.len() as isize;
        let buf_ptr = &mut buf as *mut [MaybeUninit<u8>] as *mut u8;
        let lut_ptr = DEC_DIGITS_LUT.as_ptr();

        unsafe {
            // Need at least 16 bits for the 4-characters-at-a-time to work.
            assert!(std::mem::size_of::<u64>() >= 2);

            // Eagerly decode 4 characters at a time.
            while n >= 10000 {
                let rem = (n % 10000) as isize;
                n /= 10000;

                let d1 = (rem / 100) << 1;
                let d2 = (rem % 100) << 1;
                curr -= 4;
                std::ptr::copy_nonoverlapping(lut_ptr.offset(d1), buf_ptr.offset(curr), 2);
                std::ptr::copy_nonoverlapping(lut_ptr.offset(d2), buf_ptr.offset(curr + 2), 2);
            }

            // If we reach here numbers are <= 9999, so at most 4 chars long.
            let mut n = n as isize; // Possibly reduce 64bit math.

            // Decode 2 more chars, if > 2 chars.
            if n >= 100 {
                let d1 = (n % 100) << 1;
                n /= 100;
                curr -= 2;
                std::ptr::copy_nonoverlapping(lut_ptr.offset(d1), buf_ptr.offset(curr), 2);
            }

            // Decode last 1 or 2 chars.
            if n < 10 {
                curr -= 1;
                *buf_ptr.offset(curr) = (n as u8) + b'0';
            } else {
                let d1 = n << 1;
                curr -= 2;
                std::ptr::copy_nonoverlapping(lut_ptr.offset(d1), buf_ptr.offset(curr), 2);
            }
        }

        // SAFETY: `buf` has been initialized from the `curr` offset and only contains
        //          ascii digits, thus it is valid utf8.
        let s = unsafe {
            std::str::from_utf8_unchecked(std::slice::from_raw_parts(
                buf_ptr.offset(curr),
                buf.len() - curr as usize,
            ))
        };

        serializer.serialize_str(s)
    }
}

impl<'de> Deserialize<'de> for Snowflake {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(U64Visitor).map(Snowflake)
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn test_serialize() {
        let value = json!("80351110224678912");
        let snowflake = Snowflake::from(80351110224678912);
        assert_eq!(value, serde_json::to_value(snowflake).unwrap());
    }

    #[test]
    fn test_deserialize() {
        let snowflake = Snowflake::from(80351110224678912);

        let value = json!(80351110224678912u64);
        assert_eq!(
            snowflake,
            serde_json::from_value::<Snowflake>(value).unwrap()
        );

        let value = json!("80351110224678912");
        assert_eq!(
            snowflake,
            serde_json::from_value::<Snowflake>(value).unwrap()
        );
    }
}
