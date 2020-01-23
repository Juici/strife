use serde::{Deserialize, Serialize};

use crate::model::channel::guild::PartialGuildChannel;
use crate::model::channel::permissions::PermissionOverwrite;
use crate::model::id::GuildId;

/// An organizational category that contains non-category channels.
#[non_exhaustive]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Category {
    #[serde(flatten)]
    channel: PartialGuildChannel,
    /// The ID of the guild.
    pub guild_id: GuildId,
    /// The sorting position of the chanel.
    pub position: usize,
    /// A collection of explicit permission overwrites for members and roles.
    #[serde(default)]
    pub permission_overwrites: Vec<PermissionOverwrite>,
}
wrap!(Category => mut channel: PartialGuildChannel);

impl_eq_fields!(Category: [
    channel,
    guild_id,
    position,
    permission_overwrites,
]);

#[cfg(test)]
mod tests {
    use serde_json::json;

    use crate::model::channel::{Channel, ChannelType, GuildChannel};

    use super::*;
    use crate::model::id::ChannelId;

    #[test]
    fn test_deserialize() {
        let value = json!({
          "permission_overwrites": [],
          "name": "Test",
          "parent_id": null,
          "nsfw": false,
          "position": 0,
          "guild_id": "290926798629997250",
          "type": 4,
          "id": "399942396007890945"
        });
        let channel = Category {
            channel: PartialGuildChannel {
                id: ChannelId::from(399942396007890945),
                kind: ChannelType::Category,
                name: "Test".to_owned(),
            },
            guild_id: GuildId::from(290926798629997250),
            position: 0,
            permission_overwrites: vec![],
        };

        let deserialized = Category::deserialize(&value).unwrap();
        assert_eq_fields!(channel, deserialized);

        let channel = Channel::Guild(GuildChannel::Category(channel));
        let deserialized = Channel::deserialize(&value).unwrap();
        assert_eq_fields!(channel, deserialized);
    }

    #[test]
    fn test_serialize() {
        let value = json!({
          "permission_overwrites": [],
          "name": "Test",
          "position": 0,
          "guild_id": "290926798629997250",
          "type": 4,
          "id": "399942396007890945"
        });
        let channel = Category {
            channel: PartialGuildChannel {
                id: ChannelId::from(399942396007890945),
                kind: ChannelType::Category,
                name: "Test".to_owned(),
            },
            guild_id: GuildId::from(290926798629997250),
            position: 0,
            permission_overwrites: vec![],
        };

        assert_eq!(value, serde_json::to_value(&channel).unwrap());
    }
}
