use std::fmt::{self, Display};
use std::num::ParseIntError;
use std::str::FromStr;

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use thiserror::Error;

use crate::model::utils::U16Visitor;

/// A 4-digit user discriminator tag.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Discriminator(u16);

impl Discriminator {
    /// Creates a 4-digit discriminator tag from the given value.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # use std::error::Error;
    /// use strife::model::user::Discriminator;
    ///
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// let discriminator = Discriminator::new(123)?;
    ///
    /// assert_eq!("0123".parse::<Discriminator>()?, discriminator);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// Invalid:
    ///
    /// ```
    /// use strife::model::user::Discriminator;
    ///
    /// // Value is too large to be a valid discriminator.
    /// let discriminator = 12345;
    ///
    /// assert!(Discriminator::new(discriminator).is_err());
    /// ```
    /// See the docs for [`DiscriminatorParseError`][error] for more details.
    ///
    /// [error]: enum.DiscriminatorParseError.html
    pub fn new(value: u16) -> Result<Discriminator, DiscriminatorParseError> {
        match value {
            0..=9999 => Ok(Discriminator(value)),
            value => Err(DiscriminatorParseError::Invalid(value)),
        }
    }

    /// Constructs a 4-digit discriminator tag from the given value without
    /// checking that the value is a 4-digit base-10 integer.
    ///
    /// See the safe function, [`Discriminator::new`][new], for more
    /// information.
    ///
    /// [new]: struct.Discriminator.html#method.new
    ///
    /// # Safety
    ///
    /// This function is unsafe because it does not check that `value <= 9999`.
    /// If this constraint is violated, undefined behavior results, as the
    /// rest of the library assumes that discriminator value is a 4-digit
    /// base-10 integer.
    pub unsafe fn new_unchecked(value: u16) -> Discriminator {
        Discriminator(value)
    }
}

/// An error parsing a 4-digit discriminator tag.
#[derive(Clone, Debug, Error)]
pub enum DiscriminatorParseError {
    /// An error parsing the tag.
    #[error(transparent)]
    ParseError(ParseIntError),
    /// An invalid value for the discriminator tag.
    #[error("invalid value for discriminator: {0} (must be max 9999)")]
    Invalid(u16),
}

impl FromStr for Discriminator {
    type Err = DiscriminatorParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value: u16 = s.parse().map_err(DiscriminatorParseError::ParseError)?;
        Discriminator::new(value)
    }
}

impl AsRef<u16> for Discriminator {
    fn as_ref(&self) -> &u16 {
        &self.0
    }
}

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
        let mut buf = [b'0'; 4];
        let mut n = self.0;

        let mut i = 3;
        loop {
            let digit = (n % 10) as u8;

            buf[i] = b'0' + digit;

            n /= 10;
            if n == 0 {
                break;
            }

            // Sanity checks that `i` doesn't underflow.
            i = match i {
                0 => unreachable!(),
                i => i - 1,
            };
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

#[cfg(test)]
mod tests {
    use super::*;

    use serde_json::json;

    #[test]
    fn test_parse() {
        let d = Discriminator::new(369).unwrap();
        assert_eq!(d, "0369".parse::<Discriminator>().unwrap());
        assert_eq!(d, "369".parse::<Discriminator>().unwrap());
    }

    #[test]
    fn test_new_unchecked() {
        let d1 = unsafe { Discriminator::new_unchecked(500) };

        let d2 = Discriminator::new(500).unwrap();
        let d3 = "0500".parse::<Discriminator>().unwrap();

        assert_eq!(d1, d2);
        assert_eq!(d1, d3);
    }

    #[test]
    fn test_invalid() {
        assert!(Discriminator::new(10000).is_err());
    }

    #[test]
    fn test_serialize() {
        let v = json!("0001");
        let d = Discriminator::new(1).unwrap();

        assert_eq!(v, serde_json::to_value(&d).unwrap());
    }

    #[test]
    fn test_deserialize() {
        let v = json!("0001");
        let d = Discriminator::new(1).unwrap();

        assert_eq!(d, serde_json::from_value(v).unwrap());
    }
}
