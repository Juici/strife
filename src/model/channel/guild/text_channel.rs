use chrono::{DateTime, FixedOffset};
use num_traits::Zero;
use serde::{Deserialize, Serialize};

use crate::model::channel::permissions::PermissionOverwrite;
use crate::model::channel::ChannelType;
use crate::model::id::{ChannelId, GuildId, MessageId};

/// A text channel in a [`Guild`].
///
/// [`Guild`]: ../../guild/struct.Guild.html
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
    pub position: usize,
    /// A collection of explicit permission overwrites for members and roles.
    #[serde(default)]
    pub permission_overwrites: Vec<PermissionOverwrite>,
    /// The name of the channel.
    pub name: String,
    /// The topic of the channel.
    #[serde(default)]
    pub topic: Option<String>,
    /// Whether the channel is NSFW.
    #[serde(default)]
    pub nsfw: bool,
    /// The ID of the last message sent to the group.
    #[serde(default)]
    pub last_message_id: Option<MessageId>,
    /// The amount of seconds a user has to wait before sending another message
    /// (0 - 216000s).
    ///
    /// Bots as well as users with the permission [`MANAGE_MESSAGES`] or
    /// [`MANAGE_CHANNEL`], are unaffected.
    #[doc = "\n[`MANAGE_MESSAGES`]: ../permissions/struct.Permissions.html#associatedconstant.MANAGE_MESSAGES"]
    #[doc = "\n[`MANAGE_CHANNEL`]: ../permissions/struct.Permissions.html#associatedconstant.MANAGE_CHANNEL"]
    #[serde(rename = "rate_limit_per_user")]
    #[serde(default, skip_serializing_if = "Zero::is_zero")]
    pub rate_limit: u16,
    /// The ID of the parent category of the channel.
    #[serde(default)]
    pub parent_id: Option<ChannelId>,
    /// When the last message was pinned.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_pin_timestamp: Option<DateTime<FixedOffset>>,
}

impl_eq_fields!(TextChannel: [
    id,
    kind,
    guild_id,
    position,
    permission_overwrites,
    name,
    topic,
    nsfw,
    last_message_id,
    rate_limit,
    parent_id,
    last_pin_timestamp,
]);

#[cfg(test)]
mod tests {
    use serde_json::json;

    use crate::model::channel::{Channel, GuildChannel};

    use super::*;

    #[test]
    fn test_deserialize() {
        let value = json!({
          "id": "41771983423143937",
          "guild_id": "41771983423143937",
          "name": "general",
          "type": 0,
          "position": 6,
          "permission_overwrites": [],
          "rate_limit_per_user": 2,
          "nsfw": true,
          "topic": "24/7 chat about how to gank Mike #2",
          "last_message_id": "155117677105512449",
          "parent_id": "399942396007890945"
        });
        let channel = TextChannel {
            id: ChannelId::from(41771983423143937),
            kind: ChannelType::Text,
            guild_id: GuildId::from(41771983423143937),
            position: 6,
            permission_overwrites: vec![],
            name: "general".to_owned(),
            topic: Some("24/7 chat about how to gank Mike #2".to_owned()),
            nsfw: true,
            last_message_id: Some(MessageId::from(155117677105512449)),
            rate_limit: 2,
            parent_id: Some(ChannelId::from(399942396007890945)),
            last_pin_timestamp: None,
        };

        let deserialized = TextChannel::deserialize(&value).unwrap();
        assert_eq_fields!(channel, deserialized);

        let channel = Channel::Guild(GuildChannel::Text(channel));
        let deserialized = Channel::deserialize(&value).unwrap();
        assert_eq_fields!(channel, deserialized);
    }

    #[test]
    fn test_serialize() {
        let value = json!({
          "id": "41771983423143937",
          "guild_id": "41771983423143937",
          "name": "general",
          "type": 0,
          "position": 6,
          "permission_overwrites": [],
          "rate_limit_per_user": 2,
          "nsfw": true,
          "topic": "24/7 chat about how to gank Mike #2",
          "last_message_id": "155117677105512449",
          "parent_id": "399942396007890945"
        });
        let channel = TextChannel {
            id: ChannelId::from(41771983423143937),
            kind: ChannelType::Text,
            guild_id: GuildId::from(41771983423143937),
            position: 6,
            permission_overwrites: vec![],
            name: "general".to_owned(),
            topic: Some("24/7 chat about how to gank Mike #2".to_owned()),
            nsfw: true,
            last_message_id: Some(MessageId::from(155117677105512449)),
            rate_limit: 2,
            parent_id: Some(ChannelId::from(399942396007890945)),
            last_pin_timestamp: None,
        };

        assert_eq!(value, serde_json::to_value(&channel).unwrap());
    }
}
