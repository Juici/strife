//! Models related to channels.

// Internal utility module.
mod utils;

mod attachment;
mod dm_channel;
mod embed;
mod group_channel;
mod guild_channel;
mod message;
mod permission_overwrite;
mod rich_presence;

use serde::de;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::model::guild::PartialEmoji;

pub use self::attachment::Attachment;
pub use self::dm_channel::DMChannel;
pub use self::embed::{
    Embed, EmbedAuthor, EmbedField, EmbedFooter, EmbedImage, EmbedProvider, EmbedThumbnail,
    EmbedType, EmbedVideo,
};
pub use self::group_channel::Group;
pub use self::guild_channel::GuildChannel;
pub use self::message::{
    MentionedChannel, MentionedUser, Message, MessageFlags, MessageReference, MessageType,
};
pub use self::permission_overwrite::{OverwriteId, PermissionOverwrite};
pub use self::rich_presence::{MessageActivity, MessageActivityType, MessageApplication};

/// The type of a channel.
#[non_exhaustive]
#[int_enum::int_enum(u8)]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ChannelType {
    /// A text channel in a guild.
    Text = 0,
    /// A direct message channel between the client user and another user.
    Private = 1,
    /// A voice channel in a guild.
    Voice = 2,
    /// A group message channel between multiple users.
    Group = 3,
    /// An organizational category that contains non-category channels.
    Category = 4,
    /// A channel that users can follow and crosspost into another guild.
    News = 5,
    /// A channel in which game developers can sell games on Discord.
    Store = 6,
}

impl ChannelType {
    pub(crate) fn from_map<E>(
        map: &serde_json::Map<String, serde_json::Value>,
    ) -> Result<ChannelType, E>
    where
        E: de::Error,
    {
        match map.get("type") {
            Some(kind) => match Deserialize::deserialize(kind) {
                Ok(kind) => Ok(kind),
                Err(_) => return Err(E::custom(format_args!("unknown channel type: {}", kind))),
            },
            None => return Err(E::missing_field("type")),
        }
    }
}

/// A reaction to a [`Message`].
///
/// [`Message`]: struct.Message.html
#[non_exhaustive]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Reaction {
    /// The number of reactions with this emoji.
    pub count: u64,
    /// Whether the current user reacted with this emoji.
    pub me: bool,
    /// The partial emoji information for the reaction.
    pub emoji: PartialEmoji,
}

/// A channel in Discord.
#[non_exhaustive]
#[remain::sorted]
#[derive(Clone, Debug)]
pub enum Channel {
    /// A direct message channel between the [`ClientUser`] and another
    /// [`User`].
    ///
    /// [`ClientUser`]: ../user/struct.ClientUser.html
    /// [`User`]: ../user/struct.User.html
    DM(DMChannel),
    /// A group message channel between multiple [`User`]s.
    ///
    /// [`User`]: ../user/struct.User.html
    Group(Group),
    /// A channel within a [`Guild`].
    ///
    /// [`Guild`]: TODO
    Guild(GuildChannel),
}

impl Serialize for Channel {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Channel::DM(channel) => channel.serialize(serializer),
            Channel::Group(channel) => channel.serialize(serializer),
            Channel::Guild(channel) => channel.serialize(serializer),
        }
    }
}

impl<'de> Deserialize<'de> for Channel {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde_json::{Map, Value};

        let map: Map<String, Value> = Map::deserialize(deserializer)?;
        let kind = ChannelType::from_map(&map)?;

        let value = Value::Object(map);
        match kind {
            ChannelType::Text
            | ChannelType::Voice
            | ChannelType::Category
            | ChannelType::News
            | ChannelType::Store => GuildChannel::from_value(kind, value).map(Channel::Guild),
            ChannelType::Private => DMChannel::deserialize(value)
                .map(Channel::DM)
                .map_err(de::Error::custom),
            ChannelType::Group => Group::deserialize(value)
                .map(Channel::Group)
                .map_err(de::Error::custom),
        }
    }
}

impl_eq_fields!(Channel: (a, b) => {
    match (a, b) {
        (Channel::DM(a), Channel::DM(b)) => assert_eq_fields!(a, b),
        (Channel::Group(a), Channel::Group(b)) => assert_eq_fields!(a, b),
        (Channel::Guild(a), Channel::Guild(b)) => assert_eq_fields!(a, b),
        (a, b) => panic_ne_fields!(a, b),
    }
});

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::iter::FromIterator;

    use serde_json::json;

    use crate::model::id::{ChannelId, MessageId, UserId};
    use crate::model::user::{Discriminator, User};

    use super::*;

    #[test]
    fn test_deserialize_dm() {
        let value = json!({
          "last_message_id": "3343820033257021450",
          "type": 1,
          "id": "319674150115610528",
          "recipients": [
            {
              "username": "test",
              "discriminator": "9999",
              "id": "82198898841029460",
              "avatar": "33ecab261d4681afa4d85a04691c4a01"
            }
          ]
        });
        let channel = Channel::DM(DMChannel {
            id: ChannelId::from(319674150115610528),
            kind: ChannelType::Private,
            recipient: User {
                id: UserId::from(82198898841029460),
                name: "test".to_owned(),
                discriminator: Discriminator::new(9999).unwrap(),
                avatar: Some("33ecab261d4681afa4d85a04691c4a01".to_owned()),
                bot: false,
                system: false,
            },
            last_message_id: Some(MessageId::from(3343820033257021450)),
            last_pin_timestamp: None,
        });

        let deserialized: Channel = serde_json::from_value(value).unwrap();

        assert_eq_fields!(channel, deserialized);
    }

    #[test]
    fn test_deserialize_group() {
        let value = json!({
          "name": "Some test channel",
          "icon": null,
          "recipients": [
            {
              "username": "test",
              "discriminator": "9999",
              "id": "82198898841029460",
              "avatar": "33ecab261d4681afa4d85a04691c4a01"
            },
            {
              "username": "test2",
              "discriminator": "9999",
              "id": "53908099506183680",
              "avatar": "a_bab14f271d565501444b2ca3be944b25"
            }
          ],
          "last_message_id": "3343820033257021450",
          "type": 3,
          "id": "319674150115710528",
          "owner_id": "82198810841029460"
        });
        let channel = Channel::Group(Group {
            id: ChannelId::from(319674150115710528),
            kind: ChannelType::Group,
            name: Some("Some test channel".to_owned()),
            icon: None,
            recipients: HashMap::from_iter(vec![
                (
                    UserId::from(82198898841029460),
                    User {
                        id: UserId::from(82198898841029460),
                        name: "test".to_string(),
                        discriminator: Discriminator::new(9999).unwrap(),
                        avatar: Some("33ecab261d4681afa4d85a04691c4a01".to_owned()),
                        bot: false,
                        system: false,
                    },
                ),
                (
                    UserId::from(53908099506183680),
                    User {
                        id: UserId::from(53908099506183680),
                        name: "test2".to_string(),
                        discriminator: Discriminator::new(9999).unwrap(),
                        avatar: Some("a_bab14f271d565501444b2ca3be944b25".to_owned()),
                        bot: false,
                        system: false,
                    },
                ),
            ]),
            owner_id: UserId::from(82198810841029460),
            last_message_id: Some(MessageId::from(3343820033257021450)),
            last_pin_timestamp: None,
        });

        let deserialized: Channel = serde_json::from_value(value).unwrap();

        assert_eq_fields!(channel, deserialized);
    }
}
