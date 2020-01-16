use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

use crate::model::id::{RoleId, ToSnowflakeId, UserId};
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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nick: Option<String>,
    /// When the user used their Nitro boost on the guild.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub premium_since: Option<DateTime<FixedOffset>>,
}
wrap!(Member => mut member: PartialMember);

#[doc(hidden)]
impl crate::model::id::private::Sealed for Member {}

impl ToSnowflakeId for Member {
    type Id = UserId;

    /// The ID of the channel.
    fn id(&self) -> Self::Id {
        self.user.id
    }
}

impl_to_snowflake!(Member: |member| member.id().snowflake());

/// A member of a guild, with partial information.
#[non_exhaustive]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PartialMember {
    /// The roles of the user.
    pub roles: Vec<RoleId>,
    /// When the user joined the guild.
    pub joined_at: DateTime<FixedOffset>,
    /// Whether the user in deafened in voice channels.
    pub deaf: bool,
    /// Whether the user in muted in voice channels.
    pub mute: bool,
}

impl_eq_fields!(Member: [member, user, nick, premium_since]);
impl_eq_fields!(PartialMember: [roles, joined_at, deaf, mute]);
