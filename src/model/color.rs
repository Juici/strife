//! Models a color as an integer matching the Discord API representation.

use std::fmt;

use serde::{Deserialize, Serialize};

/// A color used in the Discord API.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Deserialize, Serialize)]
pub struct Color(u32);

impl Color {
    /// Creates a new `Color` with the given integer value.
    ///
    /// # Notes
    ///
    /// Only the lowest 24-bits are taken into consideration when creating the
    /// color.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use strife::model::color::Color;
    ///
    /// let value = 0x12345678;
    /// let color = Color::new(value);
    ///
    /// assert_ne!(value, color);
    /// assert_eq!(0x345678, color);
    /// ```
    ///
    /// Oversized value:
    ///
    /// ```
    /// use strife::model::color::Color;
    ///
    /// // Oversized value, will be limited to 0x345678.
    /// let value = 0x12345678;
    /// let color = Color::new(value);
    ///
    /// assert_ne!(value, color);
    /// assert_eq!(0x345678, color);
    /// ```
    pub const fn new(value: u32) -> Color {
        Color(value & 0xFFFFFF)
    }

    /// Creates a new `Color` with the given integer value.
    ///
    /// # Safety
    ///
    /// This function is unsafe because it does not check that
    /// `value <= 0xFFFFFF`. If this constraint is violated, undefined behavior
    /// results, as the rest of the library assumes that the color value only
    /// uses the lowest 24-bits.
    pub const unsafe fn new_unchecked(value: u32) -> Color {
        Color(value)
    }

    /// Creates a new `Color` with the given RGB component values.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use strife::model::color::Color;
    ///
    /// let color = Color::rgb(0x42, 0x8C, 0x53);
    ///
    /// assert_eq!(color, Color::new(0x428C53));
    /// ```
    pub const fn rgb(red: u8, green: u8, blue: u8) -> Color {
        Color(((red as u32) << 16) | ((green as u32) << 8) | (blue as u32))
    }

    /// Returns the value of the red RGB component.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use strife::model::color::Color;
    ///
    /// let color = Color::rgb(0x42, 0x8C, 0x53);
    ///
    /// assert_eq!(color.r(), 0x42);
    /// ```
    pub const fn r(self) -> u8 {
        ((self.0 >> 16) & 0xFF) as u8
    }

    /// Returns the value of the green RGB component.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use strife::model::color::Color;
    ///
    /// let color = Color::rgb(0x42, 0x8C, 0x53);
    ///
    /// assert_eq!(color.g(), 0x8C);
    /// ```
    pub const fn g(self) -> u8 {
        ((self.0 >> 8) & 0xFF) as u8
    }

    /// Returns the value of the blue RGB component.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use strife::model::color::Color;
    ///
    /// let color = Color::rgb(0x42, 0x8C, 0x53);
    ///
    /// assert_eq!(color.b(), 0x53);
    /// ```
    pub const fn b(self) -> u8 {
        (self.0 & 0xFF) as u8
    }
}

impl Default for Color {
    fn default() -> Self {
        Color::new(0)
    }
}

impl From<u32> for Color {
    fn from(color: u32) -> Self {
        Color::new(color)
    }
}

impl From<Color> for u32 {
    fn from(color: Color) -> Self {
        color.0
    }
}

impl AsRef<u32> for Color {
    fn as_ref(&self) -> &u32 {
        &self.0
    }
}

impl PartialEq<u32> for Color {
    fn eq(&self, other: &u32) -> bool {
        self.0 == *other
    }
}

impl PartialEq<Color> for u32 {
    fn eq(&self, other: &Color) -> bool {
        other.0 == *self
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if f.alternate() {
            write!(f, "rgb({}, {}, {})", self.r(), self.g(), self.b())
        } else {
            self.0.fmt(f)
        }
    }
}

impl fmt::LowerHex for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if f.alternate() {
            f.write_str("0x")?;
        }
        write!(f, "{:06x}", self.0)
    }
}

impl fmt::UpperHex for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if f.alternate() {
            f.write_str("0x")?;
        }
        write!(f, "{:06X}", self.0)
    }
}

impl fmt::Binary for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if f.alternate() {
            f.write_str("0b")?;
        }
        write!(f, "{:024b}", self.0)
    }
}
