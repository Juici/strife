use serde::{Deserialize, Serialize};

use crate::model::channel::permissions::PermissionOverwrite;
use crate::model::channel::ChannelType;
use crate::model::id::{ChannelId, GuildId};

/// An organizational category that contains non-category channels.
#[non_exhaustive]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Category {
    /// The ID of the channel.
    pub id: ChannelId,
    /// The type of the channel.
    ///
    /// This should always be [`ChannelType::Category`].
    ///
    /// [`ChannelType::Category`]: ../enum.ChannelType.html#variant.Category
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
}

impl_eq_fields!(Category: [
    id,
    kind,
    guild_id,
    position,
    permission_overwrites,
    name,
]);

#[cfg(test)]
mod tests {
    use serde_json::json;

    use crate::model::channel::{Channel, GuildChannel};

    use super::*;

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
            id: ChannelId::from(399942396007890945),
            kind: ChannelType::Category,
            guild_id: GuildId::from(290926798629997250),
            position: 0,
            permission_overwrites: vec![],
            name: "Test".to_owned(),
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
            id: ChannelId::from(399942396007890945),
            kind: ChannelType::Category,
            guild_id: GuildId::from(290926798629997250),
            position: 0,
            permission_overwrites: vec![],
            name: "Test".to_owned(),
        };

        assert_eq!(value, serde_json::to_value(&channel).unwrap());
    }
}
