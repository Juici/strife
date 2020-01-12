//! Models related to guilds.

mod audit_log;
mod emoji;
mod member;
mod role;

use serde::{Deserialize, Serialize};

use crate::model::id::{ChannelId, GuildId, UserId};
use crate::model::utils::is_false;
use crate::model::voice::VoiceRegionId;

pub use self::audit_log::AuditLogEvent;
pub use self::emoji::{Emoji, PartialEmoji};
pub use self::member::{Member, PartialMember};
pub use self::role::Role;
use crate::model::permissions::Permissions;

/// The required level of criteria a user must meet, prior to being able to send
/// messages in a [`Guild`].
///
/// [`Guild`}: struct.Guild.html
#[non_exhaustive]
#[int_enum::int_enum(u8)]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum VerificationLevel {
    /// Does not require any verification.
    None = 0,
    /// Must have a verified email on the user's Discord account.
    Low = 1,
    /// Must also be a registered user on Discord for longer than 5 minutes.
    Medium = 2,
    /// Must also be a member of the guild for longer than 10 minutes.
    High = 3,
    /// Must have a verified phone on the user's Discord account.
    Higher = 4,
}

/// A guild with partial information.
#[non_exhaustive]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PartialGuild {
    /// The ID of the guild.
    pub id: GuildId,
    /// The name of the guild.
    pub name: String,
    /// The hash of the guild icon.
    pub icon: Option<String>,
    /// Whether the client user is the owner of the guild.
    pub owner: bool,
    /// The set of permissions for the client user in the guild (excluding
    /// channel overrides).
    pub permissions: Option<Permissions>,
}

/// A guild in Discord represents an isolated collection of users and channels,
/// and are often referred to as "servers" in the UI.
#[non_exhaustive]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Guild {
    #[serde(flatten)]
    guild: PartialGuild,
    /// The hash of the guild splash.
    pub splash: Option<String>,
    /// The ID of the owner of the guild.
    pub owner_id: UserId,
    /// The ID of the guild voice region.
    pub region: VoiceRegionId,
    /// The ID of the AFK channel.
    pub afk_channel_id: Option<ChannelId>,
    /// The AFK timeout in seconds.
    pub afk_timeout: u64,
    /// Whether the guild is embeddable (eg. widget).
    #[serde(default, skip_serializing_if = "is_false")]
    pub embed_enabled: bool,
    /// The ID of the channel that the embed widget will generate in invite to.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub embed_channel_id: Option<ChannelId>,
    /// The verification level required for the guild.
    pub verification_level: VerificationLevel,
}
wrap!(Guild => mut guild: PartialGuild);
