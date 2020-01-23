use serde::{Deserialize, Serialize};

use crate::model::channel::guild::PartialGuildChannel;
use crate::model::channel::permissions::PermissionOverwrite;
use crate::model::id::{ChannelId, GuildId};

/// A channel in which game developers can sell games on Discord.
#[non_exhaustive]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct StoreChannel {
    #[serde(flatten)]
    channel: PartialGuildChannel,
    /// The ID of the guild.
    pub guild_id: GuildId,
    /// The sorting position of the chanel.
    pub position: usize,
    /// A collection of explicit permission overwrites for members and roles.
    #[serde(default)]
    pub permission_overwrites: Vec<PermissionOverwrite>,
    /// Whether the channel is NSFW.
    #[serde(default)]
    pub nsfw: bool,
    /// The ID of the parent category of the channel.
    #[serde(default)]
    pub parent_id: Option<ChannelId>,
}
wrap!(StoreChannel => mut channel: PartialGuildChannel);

impl_eq_fields!(StoreChannel: [
    channel,
    guild_id,
    position,
    permission_overwrites,
    nsfw,
    parent_id,
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
          "name": "buy dota-2",
          "type": 6,
          "position": 0,
          "permission_overwrites": [],
          "nsfw": false,
          "parent_id": null
        });
        let channel = StoreChannel {
            channel: PartialGuildChannel {
                id: ChannelId::from(41771983423143937),
                kind: ChannelType::Store,
                name: "buy dota-2".to_owned(),
            },
            guild_id: GuildId::from(41771983423143937),
            position: 0,
            permission_overwrites: vec![],
            nsfw: false,
            parent_id: None,
        };

        let deserialized = StoreChannel::deserialize(&value).unwrap();
        assert_eq_fields!(channel, deserialized);

        let channel = Channel::Guild(GuildChannel::Store(channel));
        let deserialized = Channel::deserialize(&value).unwrap();
        assert_eq_fields!(channel, deserialized);
    }

    #[test]
    fn test_serialize() {
        let value = json!({
          "id": "41771983423143937",
          "guild_id": "41771983423143937",
          "name": "buy dota-2",
          "type": 6,
          "position": 0,
          "permission_overwrites": [],
          "nsfw": false,
          "parent_id": null
        });
        let channel = StoreChannel {
            channel: PartialGuildChannel {
                id: ChannelId::from(41771983423143937),
                kind: ChannelType::Store,
                name: "buy dota-2".to_owned(),
            },
            guild_id: GuildId::from(41771983423143937),
            position: 0,
            permission_overwrites: vec![],
            nsfw: false,
            parent_id: None,
        };

        assert_eq!(value, serde_json::to_value(&channel).unwrap());
    }
}
