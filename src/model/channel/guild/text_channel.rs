use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

use crate::model::channel::permissions::PermissionOverwrite;
use crate::model::channel::ChannelType;
use crate::model::id::{ChannelId, GuildId, MessageId};

/// A text channel in a [`Guild`].
///
/// [`Guild`]: TODO
#[non_exhaustive]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TextChannel {
    /// The ID of the channel.
    pub id: ChannelId,
    /// The type of the channel.
    ///
    /// This should always be [`ChannelType::Text`].
    ///
    /// [`ChannelType::Text`]: ../enum.ChannelType.html#variant.Text
    #[serde(rename = "type")]
    pub(crate) kind: ChannelType,
    /// The ID of the guild.
    pub guild_id: GuildId,
    /// The sorting position of the chanel.
    pub position: u64,
    /// A collection of explicit permission overwrites for members and roles.
    #[serde(default)]
    pub permission_overwrites: Vec<PermissionOverwrite>,
    /// The name of the channel.
    pub name: String,
    /// The topic of the channel.
    pub topic: Option<String>,
    /// Whether the channel is NSFW.
    #[serde(default)]
    pub nsfw: bool,
    /// The ID of the last message sent to the group.
    pub last_message_id: Option<MessageId>,
    /// The amount of seconds a user has to wait before sending another message
    /// (0 - 216000s).
    ///
    /// Bots as well as users with the permission [`MANAGE_MESSAGES`] or
    /// [`MANAGE_CHANNEL`], are unaffected.
    #[doc = "\n[`MANAGE_MESSAGES`]: ../permissions/struct.Permissions.html#associatedconstant.MANAGE_MESSAGES"]
    #[doc = "\n[`MANAGE_CHANNEL`]: ../permissions/struct.Permissions.html#associatedconstant.MANAGE_CHANNEL"]
    #[serde(default, rename = "rate_limit_per_user")]
    pub rate_limit: u16,
    /// The ID of the parent category of the channel.
    pub parent_id: Option<ChannelId>,
    /// When the last message was pinned.
    pub last_pin_timestamp: Option<DateTime<FixedOffset>>,
}
