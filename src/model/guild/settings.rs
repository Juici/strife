//! Models related to guild settings.

use int_enum::IntEnum;

/// The required level of criteria a user must meet, prior to being able to send
/// messages in a [`Guild`].
///
/// [`Guild`]: struct.Guild.html
#[repr(u8)]
#[non_exhaustive]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, IntEnum)]
pub enum VerificationLevel {
    /// Does not require any verification.
    None = 0,
    /// Must have a verified email on the user's Discord account.
    Low = 1,
    /// Must also be a registered user on Discord for longer than 5 minutes.
    Medium = 2,
    /// Must also be a member of the guild for longer than 10 minutes.
    High = 3,
    /// Must have a verified phone on the user's Discord account.
    Higher = 4,
}

impl Default for VerificationLevel {
    fn default() -> Self {
        VerificationLevel::None
    }
}

/// The default level of message notifications in a guild.
#[repr(u8)]
#[non_exhaustive]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, IntEnum)]
pub enum MessageNotificationLevel {
    /// All messages will send notifications.
    AllMessages = 0,
    /// Only messages that mention a user or a user's role will send
    /// notifications.
    OnlyMentions = 1,
}

impl Default for MessageNotificationLevel {
    fn default() -> Self {
        MessageNotificationLevel::AllMessages
    }
}

/// The level of filter to apply to users that send messages containing explicit
/// content.
#[repr(u8)]
#[non_exhaustive]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, IntEnum)]
pub enum ExplicitContentFilterLevel {
    /// No filter will be applied.
    Disabled = 0,
    /// Only members with roles will be able to send explicit content.
    MembersWithoutRoles = 1,
    /// All members will have explicit content filtered from messages they send.
    AllMembers = 2,
}

impl Default for ExplicitContentFilterLevel {
    fn default() -> Self {
        ExplicitContentFilterLevel::Disabled
    }
}

/// The required level of multi-factor authentication required for a guild.
#[repr(u8)]
#[non_exhaustive]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, IntEnum)]
pub enum MfaLevel {
    /// Multi-factor authentication is not required.
    None = 0,
    /// Multi-factor authentication is required for members to take moderation
    /// actions (eg. kick, ban, delete messages).
    Elevated = 1,
}

impl Default for MfaLevel {
    fn default() -> Self {
        MfaLevel::None
    }
}
