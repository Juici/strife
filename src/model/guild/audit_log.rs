//! Models related to audit logs.

// TODO: Docs for AuditLogEvent.

int_enum! {
    /// The [type of action] that occurred in an [`AuditLogEntry`].
    ///
    /// [type of action]: https://discordapp.com/developers/docs/resources/audit-log#audit-log-entry-object-audit-log-events
    #[allow(missing_docs)]
    #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
    pub enum AuditLogEvent: u8 {
        GuildUpdate = 1,
        ChannelCreate = 10,
        ChannelUpdate = 11,
        ChannelDelete = 12,
        ChannelOverwriteCreate = 13,
        ChannelOverwriteUpdate = 14,
        ChannelOverwriteDelete = 15,
        MemberKick = 20,
        MemberPrune = 21,
        MemberBanAdd = 22,
        MemberBanRemove = 23,
        MemberUpdate = 24,
        MemberRoleUpdate = 25,
        MemberMove = 26,
        MemberDisconnect = 27,
        BotAdd = 28,
        RoleCreate = 30,
        RoleUpdate = 31,
        RoleDelete = 32,
        InviteCreate = 40,
        InviteUpdate = 41,
        InviteDelete = 42,
        WebhookCreate = 50,
        WebhookUpdate = 51,
        WebhookDelete = 52,
        EmojiCreate = 60,
        EmojiUpdate = 61,
        EmojiDelete = 62,
        MessageDelete = 72,
        MessageBulkDelete = 73,
        MessagePin = 74,
        MessageUnpin = 75,
        IntegrationCreate = 80,
        IntegrationUpdate = 81,
        IntegrationDelete = 82,
        #[doc(hidden)]
        __Nonexhaustive,
    }
}
