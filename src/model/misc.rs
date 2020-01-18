//! Miscellaneous models.

use std::borrow::Cow;
use std::fmt::{self, Display};
use std::ops::Deref;

use serde::{Deserialize, Serialize};

/// A locale.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Deserialize, Serialize)]
#[serde(transparent)]
pub struct Locale(pub(crate) Cow<'static, str>);

impl Locale {
    /// Locale defaults to `en-US`.
    pub const DEFAULT: Locale = Locale::from_static("en-US");

    /// Creates a voice region ID from a static string.
    pub const fn from_static(id: &'static str) -> Locale {
        Locale(Cow::Borrowed(id))
    }

    /// Creates a voice region ID from a string.
    pub fn from_string<S: Into<String>>(id: S) -> Locale {
        Locale(Cow::Owned(id.into()))
    }
}

impl Default for Locale {
    fn default() -> Self {
        Locale::DEFAULT
    }
}

impl From<&'static str> for Locale {
    fn from(s: &'static str) -> Self {
        Locale(Cow::Borrowed(s))
    }
}

impl From<String> for Locale {
    fn from(s: String) -> Self {
        Locale(Cow::Owned(s))
    }
}

impl Display for Locale {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl Deref for Locale {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

impl AsRef<str> for Locale {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

impl From<Locale> for String {
    fn from(id: Locale) -> Self {
        id.0.to_string()
    }
}

impl From<Locale> for Cow<'static, str> {
    fn from(id: Locale) -> Self {
        id.0
    }
}

impl PartialEq<str> for Locale {
    fn eq(&self, other: &str) -> bool {
        self.0 == other
    }
}

impl PartialEq<String> for Locale {
    fn eq(&self, other: &String) -> bool {
        self.0 == other[..]
    }
}

impl<'a> PartialEq<Cow<'a, str>> for Locale {
    fn eq(&self, other: &Cow<'a, str>) -> bool {
        self.0[..] == other[..]
    }
}
