use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

use crate::model::id::RoleId;
use crate::model::user::User;

// TODO: Add `guild_id` field, injected by the http `Client` API.

/// A member of a guild.
#[non_exhaustive]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Member {
    #[serde(flatten)]
    member: PartialMember,
    /// The user the member represents.
    pub user: User,
    /// The nickname of the user, if one is set.
    pub nick: Option<String>,
    /// When the user used their Nitro boost on the guild.
    pub premium_since: Option<DateTime<FixedOffset>>,
}
wrap_deref!(Member => mut member: PartialMember);

/// A member of a guild, with partial information.
#[non_exhaustive]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PartialMember {
    /// Roles of the user.
    pub roles: Vec<RoleId>,
    /// When the user joined the guild.
    pub joined_at: DateTime<FixedOffset>,
    /// Whether the user in deafened in voice channels.
    pub deaf: bool,
    /// Whether the user in muted in voice channels.
    pub mute: bool,
}
