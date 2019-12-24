//! Models related to channels.

mod message;
mod permission_overwrite;

pub use self::message::{MentionedChannel, MentionedUser, Message};
pub use self::permission_overwrite::{OverwriteId, PermissionOverwrite};

/// The type of a channel.
#[non_exhaustive]
#[int_enum::int_enum(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ChannelType {
    /// A text channel in a guild.
    Text = 0,
    /// A private message channel between 2 users.
    Private = 1,
    /// A voice channel in a guild.
    Voice = 2,
    /// A group private message channel between multiple users.
    Group = 3,
    /// An organizational category that contains non-category channels.
    Category = 4,
    /// A channel that users can follow and crosspost into another server.
    News = 5,
    /// A channel in which game developers can sell games on Discord.
    Store = 6,
}
