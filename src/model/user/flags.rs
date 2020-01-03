use bitflags::bitflags;
use serde::de::{self, Deserialize, Deserializer};
use serde::ser::{Serialize, Serializer};

use crate::model::utils::U16Visitor;

bitflags! {
    /// Flags on a [`User`] account.
    ///
    /// [`User`]: struct.User.html
    #[derive(Default)]
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

impl Serialize for UserFlags {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u16(self.bits())
    }
}

impl<'de> Deserialize<'de> for UserFlags {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let bits = deserializer.deserialize_any(U16Visitor)?;
        match UserFlags::from_bits(bits) {
            Some(perms) => Ok(perms),
            None => {
                let unknown: u16 = bits & !UserFlags::all().bits();
                Err(de::Error::custom(format!(
                    "unknown user flags bits {:b} in {:b}",
                    unknown, bits
                )))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        assert_eq!(UserFlags::default(), UserFlags::empty());
    }
}
