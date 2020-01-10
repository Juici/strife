use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::model::channel::permissions::PermissionOverwrite;
use crate::model::channel::{ChannelType, Converse};
use crate::model::id::{ChannelId, GuildId};

/// A channel in which game developers can sell games on Discord.
#[non_exhaustive]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct StoreChannel {
    /// The ID of the channel.
    pub id: ChannelId,
    /// The type of the channel.
    ///
    /// This should always be [`ChannelType::Store`].
    ///
    /// [`ChannelType::Store`]: ../enum.ChannelType.html#variant.Store
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
    /// Whether the channel is NSFW.
    #[serde(default)]
    pub nsfw: bool,
    /// The ID of the parent category of the channel.
    pub parent_id: Option<ChannelId>,
}

#[async_trait]
impl Converse for StoreChannel {
    async fn channel_id(&self) -> ChannelId {
        self.id
    }

    fn channel_type(&self) -> ChannelType {
        ChannelType::Store
    }
}

impl_eq_fields!(StoreChannel: [
    id,
    kind,
    guild_id,
    position,
    permission_overwrites,
    name,
    nsfw,
    parent_id,
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
          "name": "buy dota-2",
          "type": 6,
          "position": 0,
          "permission_overwrites": [],
          "nsfw": false,
          "parent_id": null
        });
        let channel = StoreChannel {
            id: ChannelId::from(41771983423143937),
            kind: ChannelType::Store,
            guild_id: GuildId::from(41771983423143937),
            position: 0,
            permission_overwrites: vec![],
            name: "buy dota-2".to_owned(),
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
            id: ChannelId::from(41771983423143937),
            kind: ChannelType::Store,
            guild_id: GuildId::from(41771983423143937),
            position: 0,
            permission_overwrites: vec![],
            name: "buy dota-2".to_owned(),
            nsfw: false,
            parent_id: None,
        };

        assert_eq!(value, serde_json::to_value(&channel).unwrap());
    }
}
