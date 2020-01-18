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
        // Adapted from core::fmt::num impl_Display macro.
        // https://doc.rust-lang.org/src/core/fmt/num.rs.html#192-238

        use crate::internal::DEC_DIGITS_LUT;

        // Will experience undefined behaviour if the Discriminator value is > 9999.
        let mut buf = [b'0'; 4];
        let mut curr = buf.len() as isize;
        let buf_ptr = buf.as_mut_ptr();
        let lut_ptr = DEC_DIGITS_LUT.as_ptr();

        unsafe {
            // Numbers are <= 9999, so at most 4 chars long.
            let mut n = self.0 as isize; // Possibly reduce 64bit math.

            // Decode 2 chars, if > 2 chars.
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
