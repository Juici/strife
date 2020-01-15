//! Models relating voice information.

use std::borrow::Cow;
use std::fmt::{self, Display};
use std::ops::Deref;

use serde::{Deserialize, Serialize};

use crate::model::guild::Member;
use crate::model::id::{ChannelId, GuildId, UserId};
use crate::model::utils::is_false;

/// The ID of a [`VoiceRegion`].
///
/// [`VoiceRegion`]: struct.VoiceRegion.html
#[derive(Clone, Debug, Eq, Hash, PartialEq, Deserialize, Serialize)]
#[serde(transparent)]
pub struct VoiceRegionId(pub(crate) Cow<'static, str>);

impl VoiceRegionId {
    /// Creates a voice region ID from a static string.
    pub const fn from_static(id: &'static str) -> VoiceRegionId {
        VoiceRegionId(Cow::Borrowed(id))
    }

    /// Creates a voice region ID from a string.
    pub fn from_string<S: Into<String>>(id: S) -> VoiceRegionId {
        VoiceRegionId(Cow::Owned(id.into()))
    }
}

impl From<&'static str> for VoiceRegionId {
    fn from(s: &'static str) -> Self {
        VoiceRegionId(Cow::Borrowed(s))
    }
}

impl From<String> for VoiceRegionId {
    fn from(s: String) -> Self {
        VoiceRegionId(Cow::Owned(s))
    }
}

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

impl From<VoiceRegionId> for Cow<'static, str> {
    fn from(id: VoiceRegionId) -> Self {
        id.0
    }
}

impl PartialEq<str> for VoiceRegionId {
    fn eq(&self, other: &str) -> bool {
        self.0 == other
    }
}

impl PartialEq<String> for VoiceRegionId {
    fn eq(&self, other: &String) -> bool {
        self.0 == &other[..]
    }
}

impl<'a> PartialEq<Cow<'a, str>> for VoiceRegionId {
    fn eq(&self, other: &Cow<'a, str>) -> bool {
        &self.0[..] == &other[..]
    }
}

macro_rules! region_id {
    ($(
        $(#[$attr:meta])*
        const $name:ident = $id:expr;
    )*) => {
        #[allow(missing_docs)]
        impl VoiceRegionId {
            $(
                $(#[$attr])*
                pub const $name: VoiceRegionId = VoiceRegionId::from_static($id);
            )*
        }
    };
}

region_id! {
    /// Amsterdam.
    #[deprecated]
    const AMSTERDAM = "amsterdam";
    /// Brazil.
    const BRAZIL = "brazil";
    /// Dubai.
    #[deprecated]
    const DUBAI = "dubai";
    /// Central Europe.
    #[deprecated]
    const EU_CENTRAL = "eu-central";
    /// Western Europe.
    #[deprecated]
    const EU_WEST = "eu-west";
    /// Europe.
    const EUROPE = "europe";
    /// Frankfurt.
    #[deprecated]
    const FRANKFURT = "frankfurt";
    /// Hong Kong.
    const HONG_KONG = "hongkong";
    /// India.
    const INDIA = "india";
    /// Japan.
    const JAPAN = "japan";
    /// London.
    #[deprecated]
    const LONDON = "london";
    /// Russia.
    const RUSSIA = "russia";
    /// Singapore.
    const SINGAPORE = "singapore";
    /// South Africa.
    const SOUTH_AFRICA = "southafrica";
    /// Sydney.
    const SYDNEY = "sydney";
    /// US Central.
    const US_CENTRAL = "us-central";
    /// US East.
    const US_EAST = "us-east";
    /// US South.
    const US_SOUTH = "us-south";
    /// US West.
    const US_WEST = "us-west";
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

/// The voice connection state of a [`User`].
///
/// [`User`]: ../user/struct.User.html
#[non_exhaustive]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct VoiceState {
    /// The ID of the [`Guild`] the voice state is for.
    ///
    /// [`Guild`]: ../guild/struct.Guild.html
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub guild_id: Option<GuildId>,
    /// The ID of the [`Channel`] the user is connected to.
    ///
    /// [`Channel`]: ../channel/enum.Channel.html
    pub channel_id: Option<ChannelId>,
    /// The ID of the [`User`].
    ///
    /// [`User`]: ../user/struct.User.html
    pub user_id: UserId,
    /// The guild member the voice state is for.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub member: Option<Member>,
    /// The ID of the voice session.
    pub session_id: String,
    /// Whether the user is deafened by the server.
    pub deaf: bool,
    /// Whether the user is muted by the server.
    pub mute: bool,
    /// Whether the user is locally deafened.
    pub self_deaf: bool,
    /// Whether the user is locally muted.
    pub self_mute: bool,
    /// Whether the user is streaming using "Go Live".
    #[serde(default, skip_serializing_if = "is_false")]
    pub self_stream: bool,
    /// Whether the user is muted by the client user.
    pub suppress: bool,
}

impl_eq_fields!(VoiceState: [
    guild_id,
    channel_id,
    user_id,
    member,
    session_id,
    deaf,
    mute,
    self_deaf,
    self_mute,
    self_stream,
    suppress,
]);
