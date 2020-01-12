//! Models relating voice information.

use std::borrow::Cow;
use std::fmt::{self, Display};
use std::ops::Deref;

use serde::{Deserialize, Serialize};

/// The ID of a [`VoiceRegion`].
///
/// [`VoiceRegion`]: struct.VoiceRegion.html
#[derive(Clone, Debug, Eq, Hash, PartialEq, Deserialize, Serialize)]
#[serde(transparent)]
pub struct VoiceRegionId(pub(crate) Cow<'static, str>);

impl Display for VoiceRegionId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl Deref for VoiceRegionId {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

impl AsRef<str> for VoiceRegionId {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

impl From<VoiceRegionId> for String {
    fn from(id: VoiceRegionId) -> Self {
        id.0.to_string()
    }
}

/// A voice region that can be used.
#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct VoiceRegion {
    /// The unique ID for the region.
    pub id: VoiceRegionId,
    /// The name of the region.
    pub name: String,
    /// Whether the region is VIP-only.
    pub vip: bool,
    /// Whether the region is the optimal region for the client user.
    ///
    /// This is defined as the region closest to the client user.
    pub optimal: bool,
    /// Whether the region is a deprecated voice region.
    ///
    /// Deprecated regions should be avoided.
    pub deprecated: bool,
    /// Whether the region is a custom voice region (used for events, etc.).
    pub custom: bool,
}
