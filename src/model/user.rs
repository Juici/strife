//! Models related to [users].
//!
//! [users]: https://discordapp.com/developers/docs/resources/user#user-object

use std::fmt::{self, Display};

use bitflags::bitflags;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::model::id::UserId;
use crate::model::utils::U16Visitor;

bitflags! {
    /// Flags on a [`User`] account.
    ///
    /// [`User`]: struct.User.html
    pub struct UserFlags: u16 {
        /// None.
        const NONE = 0;

        /// Discord Employee.
        const DISCORD_EMPLOYEE = 1 << 0;
        /// Discord Partner.
        const DISCORD_PARTNER = 1 << 1;
        /// Discord Employee.
        const HYPESQUAD_EVENTS = 1 << 2;
        /// Bug Hunter.
        const BUG_HUNTER = 1 << 3;

        /// House Bravery.
        const HOUSE_BRAVERY = 1 << 6;
        /// House Brilliance.
        const HOUSE_BRILLIANCE = 1 << 7;
        /// House Balance.
        const HOUSE_BALANCE = 1 << 8;
        /// Early Supporter.
        const EARLY_SUPPORTER = 1 << 9;
        /// Team User.
        const TEAM_USER = 1 << 10;

        /// System.
        const SYSTEM = 1 << 12;
    }
}

int_enum! {
    /// The level of premium a [`User`] has.
    ///
    /// [`User`]: struct.User.html
    pub enum PremiumTypes: u8 {
        /// Nitro Classic.
        NitroClassic = 1,
        /// Nitro.
        Nitro = 2,
    }
}

/// A user.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct User {
    /// The ID of the user.
    pub id: UserId,
    /// The username, not unique.
    #[serde(rename = "username")]
    pub name: String,
}

/// A 4-digit user discriminator tag.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Discriminator(u16);

impl Display for Discriminator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:04}", self.0)
    }
}

impl Serialize for Discriminator {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut buf = [0u8; 4];
        let mut n = self.0;

        let mut i = 3;
        while n > 0 {
            let digit = (n % 10) as u8;

            buf[i] = b'0' + digit;

            i = i.wrapping_sub(1);
            n /= 10;
        }

        // SAFETY: `buf` only contains ascii digits, thus it is valid utf8.
        let s = unsafe { std::str::from_utf8_unchecked(&buf) };

        serializer.serialize_str(s)
    }
}

impl<'de> Deserialize<'de> for Discriminator {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(U16Visitor).map(Discriminator)
    }
}
