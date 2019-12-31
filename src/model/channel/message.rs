use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

use crate::model::channel::{Attachment, ChannelType, Embed};
use crate::model::guild::PartialMember;
use crate::model::id::{ChannelId, GuildId, MessageId, RoleId};
use crate::model::user::User;

/// A message sent in a text channel.
#[non_exhaustive]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Message {
    /// The ID of the message.
    pub id: MessageId,
    /// The ID of the [`Channel`] the message was sent in.
    pub channel_id: ChannelId,
    /// The ID of the [`Guild`] the message was sent in.
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
    pub member: Option<PartialMember>,
    /// The content of the message.
    pub content: String,
    /// When the message was sent.
    pub timestamp: DateTime<FixedOffset>,
    /// When the message was edited, if the message has been edited.
    pub edited_timestamp: Option<DateTime<FixedOffset>>,
    /// Whether the message was sent as a TTS (Text-To-Speech) message.
    pub tts: bool,
    /// Whether the message mentions everyone.
    pub mention_everyone: bool,
    /// The users specifically mentioned in the message.
    pub mentions: Vec<MentionedUser>,
    /// The roles specifically mentioned in the message.
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
