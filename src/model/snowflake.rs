//! Models the [snowflake] format used by Discord for uniquely identifiable
//! descriptors (IDs).
//!
//! These IDs are guaranteed to be unique across all of Discord, except in some
//! unique scenarios in which child objects share their parent's ID.
//!
//! [snowflake]: https://discordapp.com/developers/docs/reference#snowflakes

use std::fmt::{self, Display};

use chrono::{DateTime, TimeZone, Utc};

/// Discord epoch is the first second of 2015.
const DISCORD_EPOCH: u64 = 1_420_070_400_000;

/// A [`Snowflake`] is a 64 bit unique ID.
///
/// [`Snowflake`]: https://discordapp.com/developers/docs/reference#snowflakes
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Snowflake(u64);

impl Snowflake {
    /// Gets the timestamp of the snowflake.
    pub fn timestamp(&self) -> DateTime<Utc> {
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
