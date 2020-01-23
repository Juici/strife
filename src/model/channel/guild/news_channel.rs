use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

use crate::model::channel::guild::PartialGuildChannel;
use crate::model::channel::permissions::PermissionOverwrite;
use crate::model::id::{ChannelId, GuildId, MessageId};

/// A text channel in a [`Guild`].
///
/// [`Guild`]: ../../guild/struct.Guild.html
#[non_exhaustive]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct NewsChannel {
    #[serde(flatten)]
    channel: PartialGuildChannel,
    /// The ID of the guild.
    pub guild_id: GuildId,
    /// The sorting position of the chanel.
    pub position: usize,
    /// A collection of explicit permission overwrites for members and roles.
    #[serde(default)]
    pub permission_overwrites: Vec<PermissionOverwrite>,
    /// The topic of the channel.
    #[serde(default)]
    pub topic: Option<String>,
    /// Whether the channel is NSFW.
    #[serde(default)]
    pub nsfw: bool,
    /// The ID of the last message sent to the group.
    #[serde(default)]
    pub last_message_id: Option<MessageId>,
    /// The ID of the parent category of the channel.
    #[serde(default)]
    pub parent_id: Option<ChannelId>,
    /// When the last message was pinned.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_pin_timestamp: Option<DateTime<FixedOffset>>,
}
wrap!(NewsChannel => mut channel: PartialGuildChannel);

impl_eq_fields!(NewsChannel: [
    channel,
    guild_id,
    position,
    permission_overwrites,
    topic,
    nsfw,
    last_message_id,
    parent_id,
    last_pin_timestamp,
]);

#[cfg(test)]
mod tests {
    use serde_json::json;

    use crate::model::channel::{Channel, ChannelType, GuildChannel};

    use super::*;

    #[test]
    fn test_deserialize() {
        let value = json!({
          "id": "41771983423143937",
          "guild_id": "41771983423143937",
          "name": "important-news",
          "type": 5,
          "position": 6,
          "permission_overwrites": [],
          "nsfw": true,
          "topic": "Rumors about Half Life 3",
          "last_message_id": "155117677105512449",
          "parent_id": "399942396007890945"
        });
        let channel = NewsChannel {
            channel: PartialGuildChannel {
                id: ChannelId::from(41771983423143937),
                kind: ChannelType::News,
                name: "important-news".to_owned(),
            },
            guild_id: GuildId::from(41771983423143937),
            position: 6,
            permission_overwrites: vec![],
            topic: Some("Rumors about Half Life 3".to_owned()),
            nsfw: true,
            last_message_id: Some(MessageId::from(155117677105512449)),
            parent_id: Some(ChannelId::from(399942396007890945)),
            last_pin_timestamp: None,
        };

        let deserialized = NewsChannel::deserialize(&value).unwrap();
        assert_eq_fields!(channel, deserialized);

        let channel = Channel::Guild(GuildChannel::News(channel));
        let deserialized = Channel::deserialize(&value).unwrap();
        assert_eq_fields!(channel, deserialized);
    }

    #[test]
    fn test_serialize() {
        let value = json!({
          "id": "41771983423143937",
          "guild_id": "41771983423143937",
          "name": "important-news",
          "type": 5,
          "position": 6,
          "permission_overwrites": [],
          "nsfw": true,
          "topic": "Rumors about Half Life 3",
          "last_message_id": "155117677105512449",
          "parent_id": "399942396007890945"
        });
        let channel = NewsChannel {
            channel: PartialGuildChannel {
                id: ChannelId::from(41771983423143937),
                kind: ChannelType::News,
                name: "important-news".to_owned(),
            },
            guild_id: GuildId::from(41771983423143937),
            position: 6,
            permission_overwrites: vec![],
            topic: Some("Rumors about Half Life 3".to_owned()),
            nsfw: true,
            last_message_id: Some(MessageId::from(155117677105512449)),
            parent_id: Some(ChannelId::from(399942396007890945)),
            last_pin_timestamp: None,
        };

        assert_eq!(value, serde_json::to_value(&channel).unwrap());
    }
}
