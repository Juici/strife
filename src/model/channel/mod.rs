//! Models related to channels.

mod attachment;
mod dm_channel;
mod embed;
mod group_channel;
mod message;
mod permission_overwrite;
mod rich_presence;

use serde::{Deserialize, Serialize};

use crate::model::guild::PartialEmoji;

pub use self::attachment::Attachment;
pub use self::dm_channel::DMChannel;
pub use self::embed::{
    Embed, EmbedAuthor, EmbedField, EmbedFooter, EmbedImage, EmbedProvider, EmbedThumbnail,
    EmbedType, EmbedVideo,
};
pub use self::group_channel::GroupChannel;
pub use self::message::{
    MentionedChannel, MentionedUser, Message, MessageFlags, MessageReference, MessageType,
};
pub use self::permission_overwrite::{OverwriteId, PermissionOverwrite};
pub use self::rich_presence::{MessageActivity, MessageActivityType, MessageApplication};

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
    Group(GroupChannel),
    /// A channel within a [`Guild`].
    ///
    /// [`Guild`]: TODO
    Guild, // TODO: Add GuildChannel.
}

// TODO: Implement Deserialize and Serialize for Channel based in ChannelType.

/// The type of a channel.
#[non_exhaustive]
#[int_enum::int_enum(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
    /// A channel that users can follow and crosspost into another server.
    News = 5,
    /// A channel in which game developers can sell games on Discord.
    Store = 6,
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
