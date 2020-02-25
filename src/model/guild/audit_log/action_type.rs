use int_enum::IntEnum;

/// The [type of action] that occurred in an [`AuditLogEntry`].
///
/// [type of action]: https://discordapp.com/developers/docs/resources/audit-log#audit-log-entry-object-audit-log-events
/// [`AuditLogEntry`]: ../struct.AuditLogEntry.html
#[repr(u8)]
#[non_exhaustive]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, IntEnum)]
pub enum AuditLogEvent {
    /// The guild settings were updated.
    GuildUpdate = 1,
    /// A channel was created.
    ChannelCreate = 10,
    /// A channel was updated.
    ChannelUpdate = 11,
    /// A channel was deleted.
    ChannelDelete = 12,
    /// A channel permission overwrite was created.
    ChannelOverwriteCreate = 13,
    /// A channel permission overwrite was updated.
    ChannelOverwriteUpdate = 14,
    /// A channel permission overwrite was deleted.
    ChannelOverwriteDelete = 15,
    /// A member was kicked.
    MemberKick = 20,
    /// A member prune was executed.
    MemberPrune = 21,
    /// A member was banned.
    MemberBanAdd = 22,
    /// A member was unbanned.
    MemberBanRemove = 23,
    /// A member was updated.
    MemberUpdate = 24,
    /// A member's roles were updated.
    MemberRoleUpdate = 25,
    /// A member was moved to a voice channel.
    MemberMove = 26,
    /// A member was disconnected from a voice channel.
    MemberDisconnect = 27,
    /// A bot was added to the guild.
    BotAdd = 28,
    /// A role was created.
    RoleCreate = 30,
    /// A role was updated.
    RoleUpdate = 31,
    /// A role was deleted.
    RoleDelete = 32,
    /// An invite was created.
    InviteCreate = 40,
    /// An invite was updated.
    InviteUpdate = 41,
    /// An invite was deleted.
    InviteDelete = 42,
    /// A webhook was created.
    WebhookCreate = 50,
    /// A webhook was updated.
    WebhookUpdate = 51,
    /// A webhook was deleted.
    WebhookDelete = 52,
    /// An emoji was created.
    EmojiCreate = 60,
    /// An emoji was updated.
    EmojiUpdate = 61,
    /// An emoji was deleted.
    EmojiDelete = 62,
    /// A message was deleted.
    MessageDelete = 72,
    /// Messages were bulk deleted.
    MessageBulkDelete = 73,
    /// A message was pinned.
    MessagePin = 74,
    /// A message was unpinned.
    MessageUnpin = 75,
    /// An integration was created.
    IntegrationCreate = 80,
    /// An integration was updated.
    IntegrationUpdate = 81,
    /// An integration was deleted.
    IntegrationDelete = 82,
}
