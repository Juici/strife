//! Message related models.

pub mod embed;

mod attachment;
mod rich_presence;

use bitflags::bitflags;
use chrono::{DateTime, FixedOffset};
use serde::de;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::model::channel::ChannelType;
use crate::model::emoji::PartialEmoji;
use crate::model::guild::PartialMember;
use crate::model::id::{ChannelId, GuildId, MessageId, RoleId, WebhookId};
use crate::model::user::User;
use crate::model::utils::U8Visitor;

use self::embed::Embed;

pub use self::attachment::Attachment;
pub use self::rich_presence::{MessageActivity, MessageActivityType, MessageApplication};

/// A message sent in a text channel.
#[non_exhaustive]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Message {
    /// The ID of the message.
    pub id: MessageId,
    /// The ID of the [`Channel`] the message was sent in.
    ///
    /// [`Channel`]: ../enum.Channel.html
    pub channel_id: ChannelId,
    /// The ID of the [`Guild`] the message was sent in.
    ///
    /// [`Guild`]: ../../guild/struct.Guild.html
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub guild_id: Option<GuildId>,
    /// The author of the message.
    ///
    /// May not be a valid user, see notes for more information.
    ///
    /// # Notes
    ///
    /// The author object follows the structure of the user object, but is only
    /// a valid user in the case where the message is generated by a user or bot
    /// user. If the message is generated by a webhook, the author object
    /// corresponds to the webhook's id, username, and avatar. You can tell if a
    /// message is generated by a webhook by checking for the webhook_id on the
    /// message object.
    pub author: User,
    /// Partial guild member properties for the [`author`], if the message was
    /// sent in a guild.
    ///
    /// [`author`]: #structfield.author
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub member: Option<PartialMember>,
    /// The content of the message.
    pub content: String,
    /// When the message was sent.
    pub timestamp: DateTime<FixedOffset>,
    /// When the message was edited, if the message has been edited.
    #[serde(default)]
    pub edited_timestamp: Option<DateTime<FixedOffset>>,
    /// Whether the message was sent as a TTS (Text-To-Speech) message.
    pub tts: bool,
    /// Whether the message mentions everyone.
    pub mention_everyone: bool,
    /// The users specifically mentioned in the message.
    #[serde(default)]
    pub mentions: Vec<MentionedUser>,
    /// The roles specifically mentioned in the message.
    #[serde(default)]
    pub mention_roles: Vec<RoleId>,
    /// The channels specifically mentioned in the message.
    ///
    /// Only textual channels that are visible to everyone in a lurkable guild
    /// will ever be included.
    #[serde(default)]
    pub mention_channels: Vec<MentionedChannel>,
    /// The files attached to the message.
    #[serde(default)]
    pub attachments: Vec<Attachment>,
    /// The embedded content of the message.
    #[serde(default)]
    pub embeds: Vec<Embed>,
    /// The reactions to the message.
    #[serde(default)]
    pub reactions: Vec<Reaction>,
    /// Whether the message is pinned.
    pub pinned: bool,
    /// The webhook ID that generated the message, if the message was generated
    /// by a webhook.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub webhook_id: Option<WebhookId>,
    /// The type of message.
    #[serde(rename = "type")]
    pub kind: MessageType,
    /// The Rich Presence activity information sent with related embeds.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub activity: Option<MessageActivity>,
    /// The Rich Presence application information sent with related embeds.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub application: Option<MessageApplication>,
    /// The reference data sent with a crossposted message.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message_reference: Option<MessageReference>,
    /// Flags describing extra features of the message.
    #[serde(default, skip_serializing_if = "MessageFlags::is_empty")]
    pub flags: MessageFlags,
}

/// Type of a [`Message`].
///
/// [`Message`]: struct.Message.html
// TODO: Add docs.
#[allow(missing_docs)]
#[non_exhaustive]
#[int_enum::int_enum(u8)]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum MessageType {
    Default = 0,
    RecipientAdd = 1,
    RecipientRemove = 2,
    Call = 3,
    ChannelNameChange = 4,
    ChannelIconChange = 5,
    ChannelPinnedMessage = 6,
    GuildMemberJoin = 7,
    UserPremiumGuildSubscription = 8,
    UserPremiumGuildSubscriptionTier1 = 9,
    UserPremiumGuildSubscriptionTier2 = 10,
    UserPremiumGuildSubscriptionTier3 = 11,
    ChannelFollowAdd = 12,
}

impl Default for MessageType {
    fn default() -> Self {
        MessageType::Default
    }
}

/// Reference data send with a crossposted [`Message`]
///
/// [`Message`]: struct.Message.html
#[non_exhaustive]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Deserialize, Serialize)]
pub struct MessageReference {
    /// The ID of the originating message.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message_id: Option<MessageId>,
    /// The ID of the originating channel.
    pub channel_id: ChannelId,
    /// The ID of the originating guild.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub guild_id: Option<GuildId>,
}

/// A user specifically mentioned in a [`Message`].
///
/// [`Message`]: struct.Message.html
#[non_exhaustive]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MentionedUser {
    #[serde(flatten)]
    user: User,
    /// Partial guild member properties for the user, if the mention was in a
    /// message sent in a guild.
    pub member: Option<PartialMember>,
}
wrap!(MentionedUser => mut user: User);

/// A textual channel specifically mentioned in a [`Message`].
///
/// [`Message`]: struct.Message.html
#[non_exhaustive]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MentionedChannel {
    /// The ID of the channel.
    pub id: ChannelId,
    /// The ID of the guild containing the channel.
    pub guild_id: GuildId,
    /// The type of the channel.
    #[serde(rename = "type")]
    pub kind: ChannelType,
    /// The name of the channel.
    pub name: String,
}

bitflags! {
    /// Flags the describe extra features on a [`Message`].
    ///
    /// [`Message`]: struct.Message.html
    #[derive(Default)]
    pub struct MessageFlags: u8 {
        /// The message has been published to subscribed channels (via Channel Following).
        const CROSSPOSTED = 1 << 0;
        /// The message originated from a message in another channel (via Channel Following).
        const IS_CROSSPOST = 1 << 1;
        /// Do not include any embeds when serializing the message.
        const SUPPRESS_EMBEDS = 1 << 2;
        /// The source message for this crossposted message has been deleted.
        const SOURCE_MESSAGE_DELETED = 1 << 3;
        /// The message came from the urgent message system.
        const URGENT = 1 << 4;
    }
}

impl Serialize for MessageFlags {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u8(self.bits())
    }
}

impl<'de> Deserialize<'de> for MessageFlags {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let bits = deserializer.deserialize_any(U8Visitor)?;
        match MessageFlags::from_bits(bits) {
            Some(perms) => Ok(perms),
            None => {
                let unknown: u8 = bits & !MessageFlags::all().bits();
                Err(de::Error::custom(format!(
                    "unknown user flags bits {:b} in {:b}",
                    unknown, bits
                )))
            }
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

impl_eq_fields!(Message: [
    id,
    channel_id,
    guild_id,
    author,
    member,
    content,
    timestamp,
    edited_timestamp,
    tts,
    mention_everyone,
    mentions,
    mention_roles,
    mention_channels,
    attachments,
    embeds,
    reactions,
    pinned,
    webhook_id,
    kind,
    activity,
    application,
    message_reference,
    flags,
]);
impl_eq_fields!(MentionedUser: [user, member]);
impl_eq_fields!(MentionedChannel: [id, guild_id, kind, name]);
impl_eq_fields!(Reaction: [count, me, emoji]);

#[cfg(test)]
mod tests {
    use serde_json::json;

    use crate::model::id::UserId;

    use super::*;

    #[test]
    fn test_deserialize_message() {
        let value = json!({
          "reactions": [
            {
              "count": 1,
              "me": false,
              "emoji": {
                "id": null,
                "name": "🔥"
              }
            }
          ],
          "attachments": [],
          "tts": false,
          "embeds": [],
          "timestamp": "2017-07-11T17:27:07.299000+00:00",
          "mention_everyone": false,
          "id": "334385199974967042",
          "pinned": false,
          "edited_timestamp": null,
          "author": {
            "username": "Mason",
            "discriminator": "9999",
            "id": "53908099506183680",
            "avatar": "a_bab14f271d565501444b2ca3be944b25"
          },
          "mention_roles": [],
          "content": "Supa Hot",
          "channel_id": "290926798999357250",
          "mentions": [],
          "type": 0
        });
        let message = Message {
            id: MessageId::from(334385199974967042),
            channel_id: ChannelId::from(290926798999357250),
            guild_id: None,
            author: User {
                id: UserId::from(53908099506183680),
                name: "Mason".to_owned(),
                discriminator: "9999".parse().unwrap(),
                avatar: Some("a_bab14f271d565501444b2ca3be944b25".to_owned()),
                bot: false,
                system: false,
            },
            member: None,
            content: "Supa Hot".to_owned(),
            timestamp: "2017-07-11T17:27:07.299000+00:00".parse().unwrap(),
            edited_timestamp: None,
            tts: false,
            mention_everyone: false,
            mentions: vec![],
            mention_roles: vec![],
            mention_channels: vec![],
            attachments: vec![],
            embeds: vec![],
            reactions: vec![Reaction {
                count: 1,
                me: false,
                emoji: PartialEmoji::standard("🔥"),
            }],
            pinned: false,
            webhook_id: None,
            kind: MessageType::Default,
            activity: None,
            application: None,
            message_reference: None,
            flags: MessageFlags::default(),
        };

        let deserialized = Message::deserialize(&value).unwrap();
        assert_eq_fields!(message, deserialized);
    }

    #[test]
    fn test_serialize_message() {
        let value = json!({
          "reactions": [
            {
              "count": 1,
              "me": false,
              "emoji": {
                "id": null,
                "name": "🔥"
              }
            }
          ],
          "attachments": [],
          "tts": false,
          "embeds": [],
          "timestamp": "2017-07-11T17:27:07.299+00:00",
          "mention_channels": [],
          "mention_everyone": false,
          "id": "334385199974967042",
          "pinned": false,
          "edited_timestamp": null,
          "author": {
            "username": "Mason",
            "discriminator": "9999",
            "id": "53908099506183680",
            "avatar": "a_bab14f271d565501444b2ca3be944b25"
          },
          "mention_roles": [],
          "content": "Supa Hot",
          "channel_id": "290926798999357250",
          "mentions": [],
          "type": 0
        });
        let message = Message {
            id: MessageId::from(334385199974967042),
            channel_id: ChannelId::from(290926798999357250),
            guild_id: None,
            author: User {
                id: UserId::from(53908099506183680),
                name: "Mason".to_owned(),
                discriminator: "9999".parse().unwrap(),
                avatar: Some("a_bab14f271d565501444b2ca3be944b25".to_owned()),
                bot: false,
                system: false,
            },
            member: None,
            content: "Supa Hot".to_owned(),
            timestamp: "2017-07-11T17:27:07.299+00:00".parse().unwrap(),
            edited_timestamp: None,
            tts: false,
            mention_everyone: false,
            mentions: vec![],
            mention_roles: vec![],
            mention_channels: vec![],
            attachments: vec![],
            embeds: vec![],
            reactions: vec![Reaction {
                count: 1,
                me: false,
                emoji: PartialEmoji::standard("🔥"),
            }],
            pinned: false,
            webhook_id: None,
            kind: MessageType::Default,
            activity: None,
            application: None,
            message_reference: None,
            flags: MessageFlags::default(),
        };

        assert_eq!(value, serde_json::to_value(&message).unwrap());
    }

    #[test]
    fn test_deserialize_crossposted_message() {
        let value = json!({
          "reactions": [
            {
              "count": 1,
              "me": false,
              "emoji": {
                "id": null,
                "name": "🔥"
              }
            }
          ],
          "attachments": [],
          "tts": false,
          "embeds": [],
          "timestamp": "2017-07-11T17:27:07.299+00:00",
          "mention_everyone": false,
          "id": "334385199974967042",
          "pinned": false,
          "edited_timestamp": null,
          "author": {
            "username": "Mason",
            "discriminator": "9999",
            "id": "53908099506183680",
            "avatar": "a_bab14f271d565501444b2ca3be944b25"
          },
          "mention_roles": [],
          "mention_channels": [
            {
              "id": "278325129692446722",
              "guild_id": "278325129692446720",
              "name": "big-news",
              "type": 5
            }
          ],
          "content": "Big news! In this <#278325129692446722> channel!",
          "channel_id": "290926798999357250",
          "mentions": [],
          "type": 0,
          "flags": 2,
          "message_reference": {
            "channel_id": "278325129692446722",
            "guild_id": "278325129692446720",
            "message_id": "306588351130107906"
          }
        });
        let message = Message {
            id: MessageId::from(334385199974967042),
            channel_id: ChannelId::from(290926798999357250),
            guild_id: None,
            author: User {
                id: UserId::from(53908099506183680),
                name: "Mason".to_owned(),
                discriminator: "9999".parse().unwrap(),
                avatar: Some("a_bab14f271d565501444b2ca3be944b25".to_owned()),
                bot: false,
                system: false,
            },
            member: None,
            content: "Big news! In this <#278325129692446722> channel!".to_owned(),
            timestamp: "2017-07-11T17:27:07.299+00:00".parse().unwrap(),
            edited_timestamp: None,
            tts: false,
            mention_everyone: false,
            mentions: vec![],
            mention_roles: vec![],
            mention_channels: vec![MentionedChannel {
                id: ChannelId::from(278325129692446722),
                guild_id: GuildId::from(278325129692446720),
                kind: ChannelType::News,
                name: "big-news".to_owned(),
            }],
            attachments: vec![],
            embeds: vec![],
            reactions: vec![Reaction {
                count: 1,
                me: false,
                emoji: PartialEmoji::standard("🔥"),
            }],
            pinned: false,
            webhook_id: None,
            kind: MessageType::Default,
            activity: None,
            application: None,
            message_reference: Some(MessageReference {
                message_id: Some(MessageId::from(306588351130107906)),
                channel_id: ChannelId::from(278325129692446722),
                guild_id: Some(GuildId::from(278325129692446720)),
            }),
            flags: MessageFlags::IS_CROSSPOST,
        };

        let deserialized = Message::deserialize(&value).unwrap();
        assert_eq_fields!(message, deserialized);
    }

    #[test]
    fn test_serialize_crossposted_message() {
        let value = json!({
          "reactions": [
            {
              "count": 1,
              "me": false,
              "emoji": {
                "id": null,
                "name": "🔥"
              }
            }
          ],
          "attachments": [],
          "tts": false,
          "embeds": [],
          "timestamp": "2017-07-11T17:27:07.299+00:00",
          "mention_everyone": false,
          "id": "334385199974967042",
          "pinned": false,
          "edited_timestamp": null,
          "author": {
            "username": "Mason",
            "discriminator": "9999",
            "id": "53908099506183680",
            "avatar": "a_bab14f271d565501444b2ca3be944b25"
          },
          "mention_roles": [],
          "mention_channels": [
            {
              "id": "278325129692446722",
              "guild_id": "278325129692446720",
              "name": "big-news",
              "type": 5
            }
          ],
          "content": "Big news! In this <#278325129692446722> channel!",
          "channel_id": "290926798999357250",
          "mentions": [],
          "type": 0,
          "flags": 2,
          "message_reference": {
            "channel_id": "278325129692446722",
            "guild_id": "278325129692446720",
            "message_id": "306588351130107906"
          }
        });
        let message = Message {
            id: MessageId::from(334385199974967042),
            channel_id: ChannelId::from(290926798999357250),
            guild_id: None,
            author: User {
                id: UserId::from(53908099506183680),
                name: "Mason".to_owned(),
                discriminator: "9999".parse().unwrap(),
                avatar: Some("a_bab14f271d565501444b2ca3be944b25".to_owned()),
                bot: false,
                system: false,
            },
            member: None,
            content: "Big news! In this <#278325129692446722> channel!".to_owned(),
            timestamp: "2017-07-11T17:27:07.299+00:00".parse().unwrap(),
            edited_timestamp: None,
            tts: false,
            mention_everyone: false,
            mentions: vec![],
            mention_roles: vec![],
            mention_channels: vec![MentionedChannel {
                id: ChannelId::from(278325129692446722),
                guild_id: GuildId::from(278325129692446720),
                kind: ChannelType::News,
                name: "big-news".to_owned(),
            }],
            attachments: vec![],
            embeds: vec![],
            reactions: vec![Reaction {
                count: 1,
                me: false,
                emoji: PartialEmoji::standard("🔥"),
            }],
            pinned: false,
            webhook_id: None,
            kind: MessageType::Default,
            activity: None,
            application: None,
            message_reference: Some(MessageReference {
                message_id: Some(MessageId::from(306588351130107906)),
                channel_id: ChannelId::from(278325129692446722),
                guild_id: Some(GuildId::from(278325129692446720)),
            }),
            flags: MessageFlags::IS_CROSSPOST,
        };

        assert_eq!(value, serde_json::to_value(&message).unwrap());
    }
}
