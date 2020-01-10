use serde::{Deserialize, Serialize};

use crate::model::channel::permissions::PermissionOverwrite;
use crate::model::channel::ChannelType;
use crate::model::id::{ChannelId, GuildId};

/// A voice channel in a [`Guild`].
///
/// [`Guild`]: TODO
#[non_exhaustive]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct VoiceChannel {
    /// The ID of the channel.
    pub id: ChannelId,
    /// The type of the channel.
    ///
    /// This should always be [`ChannelType::Voice`].
    ///
    /// [`ChannelType::Voice`]: ../enum.ChannelType.html#variant.Voice
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
    /// The bitrate (in bits) of the voice channel.
    pub bitrate: u32,
    /// The user limit of the channel voice channel, a limit of `0` is
    /// unlimited.
    pub user_limit: u8,
    /// The ID of the parent category of the channel.
    pub parent_id: Option<ChannelId>,
}

impl_eq_fields!(VoiceChannel: [
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
          "id": "155101607195836416",
          "guild_id": "41771983423143937",
          "name": "ROCKET CHEESE",
          "type": 2,
          "nsfw": false,
          "position": 5,
          "permission_overwrites": [],
          "bitrate": 64000,
          "user_limit": 0,
          "parent_id": null
        });
        let channel = VoiceChannel {
            id: ChannelId::from(155101607195836416),
            kind: ChannelType::Voice,
            guild_id: GuildId::from(41771983423143937),
            position: 5,
            permission_overwrites: vec![],
            name: "ROCKET CHEESE".to_owned(),
            bitrate: 64000,
            user_limit: 0,
            parent_id: None,
        };

        let deserialized = VoiceChannel::deserialize(&value).unwrap();
        assert_eq_fields!(channel, deserialized);

        let channel = Channel::Guild(GuildChannel::Voice(channel));
        let deserialized = Channel::deserialize(&value).unwrap();
        assert_eq_fields!(channel, deserialized);
    }

    #[test]
    fn test_serialize() {
        let value = json!({
          "id": "155101607195836416",
          "guild_id": "41771983423143937",
          "name": "ROCKET CHEESE",
          "type": 2,
          "position": 5,
          "permission_overwrites": [],
          "bitrate": 64000,
          "user_limit": 0,
          "parent_id": null
        });
        let channel = VoiceChannel {
            id: ChannelId::from(155101607195836416),
            kind: ChannelType::Voice,
            guild_id: GuildId::from(41771983423143937),
            position: 5,
            permission_overwrites: vec![],
            name: "ROCKET CHEESE".to_owned(),
            bitrate: 64000,
            user_limit: 0,
            parent_id: None,
        };

        assert_eq!(value, serde_json::to_value(&channel).unwrap());
    }
}
