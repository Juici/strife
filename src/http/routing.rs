//! Routing for the Discord REST API.
//!
//! Not part of the public API.

use std::borrow::Cow;
use std::fmt::Write;

use hyper::Method as HttpMethod;

use crate::model::channel::permissions::OverwriteId;
use crate::model::guild::{AuditLogEvent, Emoji};
use crate::model::id::*;

/// Buckets grouping [rate limited] routes.
///
/// [rate limited]: https://discordapp.com/developers/docs/topics/rate-limits#rate-limits
#[non_exhaustive]
#[remain::sorted]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Bucket {
    /// Route:
    /// ```text
    /// /channels/{channel.id}
    /// ```
    ChannelsId(ChannelId),
    /// Route:
    /// ```text
    /// /channels/{channel.id}/invites
    /// ```
    ChannelsIdInvites(ChannelId),
    /// Route:
    /// ```text
    /// /channels/{channel.id}/messages
    /// ```
    ChannelsIdMessages(ChannelId),
    /// Route:
    /// ```text
    /// /channels/{channel.id}/messages/bulk-delete
    /// ```
    ChannelsIdMessagesBulkDelete(ChannelId),
    /// Route:
    /// ```text
    /// /channels/{channel.id}/messages/{message.id}
    /// ```
    ChannelsIdMessagesId(ChannelId),
    /// Route:
    /// ```text
    /// /channels/{channel.id}/messages/{message.id}
    /// ```
    ///
    /// This has a separate bucket from [`ChannelsIdMessagesId`]. From the
    /// Discord docs:
    ///
    /// > Deleting messages falls under a separate, higher rate limit so that
    /// > bots are able to more quickly delete content from channels (which is
    /// > useful for moderation bots).
    ///
    /// [`ChannelsIdMessagesId`]: enum.Bucket.html#variant.ChannelsIdMessagesId
    ChannelsIdMessagesIdDelete(ChannelId),
    /// Route:
    /// ```text
    /// /channels/{channel.id}/messages/{message.id}/reactions
    /// ```
    ChannelsIdMessagesIdReactions(ChannelId),
    /// Route:
    /// ```text
    /// /channels/{channel.id}/messages/{message.id}/reactions/{emoji}
    /// ```
    ChannelsIdMessagesIdReactionsEmoji(ChannelId),
    /// Routes:
    /// ```text
    /// /channels/{channel.id}/messages/{message.id}/reactions/{emoji}/@me
    /// /channels/{channel.id}/messages/{message.id}/reactions/{emoji}/{user.id}
    /// ```
    ChannelsIdMessagesIdReactionsEmojiUserId(ChannelId),
    /// Route:
    /// ```text
    /// /channels/{channel.id}/permissions/{overwrite.id}
    /// ```
    ChannelsIdPermissionsOverwriteId(ChannelId),
    /// Route:
    /// ```text
    /// /channels/{channel.id}/pins
    /// ```
    ChannelsIdPins(ChannelId),
    /// Route:
    /// ```text
    /// /channels/{channel.id}/pins/{message.id}
    /// ```
    ChannelsIdPinsMessageId(ChannelId),
    /// Route:
    /// ```text
    /// /channels/{channel.id}/recipients/{user.id}
    /// ```
    ChannelsIdRecipientsUserId(ChannelId),
    /// Route:
    /// ```text
    /// /channels/{channel.id}/typing
    /// ```
    ChannelsIdTyping(ChannelId),
    /// Route:
    /// ```text
    /// /channels/{channel.id}/webhooks
    /// ```
    ChannelsIdWebhooks(ChannelId),

    /// Route:
    /// ```text
    /// /gateway
    /// ```
    Gateway,
    /// Route:
    /// ```text
    /// /gateway/bot
    /// ```
    GatewayBot,
    /// Route:
    /// ```text
    /// /guilds
    /// ```
    Guilds,
    /// Route:
    /// ```text
    /// /guilds/{guild.id}
    /// ```
    GuildsId(GuildId),
    /// Route:
    /// ```text
    /// /guild/{guild.id}/audit-logs
    /// ```
    GuildsIdAuditLogs(GuildId),
    /// Route:
    /// ```text
    /// /guilds/{guild.id}/bans
    /// ```
    GuildsIdBans(GuildId),
    /// Route:
    /// ```text
    /// /guilds/{guild.id}/bans/{user.id}
    /// ```
    GuildsIdBansUserId(GuildId),
    /// Route:
    /// ```text
    /// /guilds/{guild.id}/channels
    /// ```
    GuildsIdChannels(GuildId),
    /// Route:
    /// ```text
    /// /guilds/{guild.id}/embed
    /// ```
    GuildsIdEmbed(GuildId),
    /// Route:
    /// ```text
    /// /guilds/{guild.id}/emojis
    /// ```
    GuildsIdEmojis(GuildId),
    /// Route:
    /// ```text
    /// /guilds/{guild.id}/emojis/{emoji.id}
    /// ```
    GuildsIdEmojisId(GuildId),
    /// Route:
    /// ```text
    /// /guilds/{guild.id}/integrations
    /// ```
    GuildsIdIntegrations(GuildId),
    /// Route:
    /// ```text
    /// /guilds/{guild.id}/integrations/{integration.id}
    /// ```
    GuildsIdIntegrationsId(GuildId),
    /// Route:
    /// ```text
    /// /guilds/{guild.id}/integrations/{integration.id}/sync
    /// ```
    GuildsIdIntegrationsIdSync(GuildId),
    /// Route:
    /// ```text
    /// /guilds/{guild.id}/invites
    /// ```
    GuildsIdInvites(GuildId),
    /// Route:
    /// ```text
    /// /guilds/{guild.id}/members
    /// ```
    GuildsIdMembers(GuildId),
    /// Route:
    /// ```text
    /// /guilds/{guild.id}/members/{user.id}
    /// ```
    GuildsIdMembersId(GuildId),
    /// Route:
    /// ```text
    /// /guilds/{guild.id}/members/{user.id}/roles/{role.id}
    /// ```
    GuildsIdMembersIdRolesId(GuildId),
    /// Route:
    /// ```text
    /// /guilds/{guild.id}/members/@me/nick
    /// ```
    GuildsIdMembersMeNick(GuildId),
    /// Route:
    /// ```text
    /// /guilds/{guild.id}/prune
    /// ```
    GuildsIdPrune(GuildId),
    /// Route:
    /// ```text
    /// /guilds/{guild.id}/regions
    /// ```
    GuildsIdRegions(GuildId),
    /// Route:
    /// ```text
    /// /guilds/{guild.id}/roles
    /// ```
    GuildsIdRoles(GuildId),
    /// Route:
    /// ```text
    /// /guilds/{guild.id}/roles/{role.id}
    /// ```
    GuildsIdRolesId(GuildId),
    /// Route:
    /// ```text
    /// /guilds/{guild.id}/vanity-url
    /// ```
    GuildsIdVanityUrl(GuildId),
    /// Route:
    /// ```text
    /// /guilds/{guild.id}/webhooks
    /// ```
    GuildsIdWebhooks(GuildId),

    /// Route:
    /// ```text
    /// /invites/{invite.code}
    /// ```
    InvitesCode,

    /// Route:
    /// ```text
    /// /users/@me
    /// /users/{user.id}
    /// ```
    UsersId,
    /// Route:
    /// ```text
    /// /users/@me/channels
    /// ```
    UsersMeChannels,
    /// Route:
    /// ```text
    /// /users/@me/guilds
    /// ```
    UsersMeGuilds,
    /// Route:
    /// ```text
    /// /users/@me/guilds/{guild.id}
    /// ```
    UsersMeGuildsId(GuildId),

    /// Route:
    /// ```text
    /// /voice/regions
    /// ```
    VoiceRegions,

    /// Route:
    /// ```text
    /// /webhooks/{webhook.id}
    /// ```
    WebhooksId(WebhookId),
    /// Route:
    /// ```text
    /// /webhooks/{webhook.id}/{webhook.token}
    /// ```
    WebhooksIdToken(WebhookId),

    /// Routes where no rate limits are in place.
    #[remain::unsorted]
    None,
}

// TODO: Add support for status api (https://status.discordapp.com/api/).
/// An API endpoint.
///
/// # Stability
///
/// This is not part of the stable API and may change at any time. For a stable
/// API use the functions on the [`Http`] client.
///
/// [`Http`]: ../struct.Http.html
#[allow(missing_docs)]
#[non_exhaustive]
#[remain::sorted]
#[derive(Clone, Debug)]
pub enum Route<'a> {
    AddGroupRecipient {
        channel_id: ChannelId,
        user_id: UserId,
    },
    AddMemberRole {
        guild_id: GuildId,
        user_id: UserId,
        role_id: RoleId,
    },
    BanMember {
        guild_id: GuildId,
        user_id: UserId,
        delete_message_days: Option<u8>,
        reason: Option<&'a str>,
    },
    BroadcastTyping {
        channel_id: ChannelId,
    },
    CreateChannel {
        guild_id: GuildId,
    },
    CreateChannelWebhook {
        channel_id: ChannelId,
    },
    CreateEmoji {
        guild_id: GuildId,
    },
    CreateGuild,
    CreateIntegration {
        guild_id: GuildId,
    },
    CreateInvite {
        channel_id: ChannelId,
    },
    CreateMessage {
        channel_id: ChannelId,
    },
    CreatePrivateChannel,
    CreateReaction {
        channel_id: ChannelId,
        message_id: MessageId,
        emoji: Emoji,
    },
    CreateRole {
        guild_id: GuildId,
    },
    DeleteChannel {
        channel_id: ChannelId,
    },
    DeleteChannelPermission {
        channel_id: ChannelId,
        overwrite_id: OverwriteId,
    },
    DeleteEmoji {
        guild_id: GuildId,
        emoji_id: EmojiId,
    },
    DeleteGuild {
        guild_id: GuildId,
    },
    DeleteIntegration {
        guild_id: GuildId,
        integration_id: IntegrationId,
    },
    DeleteInvite {
        code: &'a str,
    },
    DeleteMessage {
        channel_id: ChannelId,
        message_id: MessageId,
    },
    DeleteMessagesBulk {
        channel_id: ChannelId,
    },
    DeleteOwnReaction {
        channel_id: ChannelId,
        message_id: MessageId,
        emoji: Emoji,
    },
    DeleteReaction {
        channel_id: ChannelId,
        message_id: MessageId,
        emoji: Emoji,
        user_id: UserId,
    },
    DeleteReactions {
        channel_id: ChannelId,
        message_id: MessageId,
    },
    DeleteRole {
        guild_id: GuildId,
        role_id: RoleId,
    },
    DeleteWebhook {
        webhook_id: WebhookId,
    },
    DeleteWebhookWithToken {
        webhook_id: WebhookId,
        token: &'a str,
    },
    EditChannel {
        channel_id: ChannelId,
    },
    EditChannelPermission {
        channel_id: ChannelId,
        overwrite_id: OverwriteId,
    },
    EditChannelPositions {
        guild_id: GuildId,
    },
    EditCurrentUser,
    EditEmoji {
        guild_id: GuildId,
        emoji_id: EmojiId,
    },
    EditGuild {
        guild_id: GuildId,
    },
    EditGuildEmbed {
        guild_id: GuildId,
    },
    EditIntegration {
        guild_id: GuildId,
        integration_id: IntegrationId,
    },
    EditMember {
        guild_id: GuildId,
        user_id: UserId,
    },
    EditMessage {
        channel_id: ChannelId,
        message_id: MessageId,
    },
    EditNickname {
        guild_id: GuildId,
    },
    EditRole {
        guild_id: GuildId,
        role_id: RoleId,
    },
    EditRolePositions {
        guild_id: GuildId,
    },
    EditWebhook {
        webhook_id: WebhookId,
    },
    EditWebhookWithToken {
        webhook_id: WebhookId,
        token: &'a str,
    },
    ExecuteWebhook {
        webhook_id: WebhookId,
        token: &'a str,
        wait: Option<bool>,
    },
    GetAuditLogs {
        guild_id: GuildId,
        user_id: Option<UserId>,
        action_type: Option<AuditLogEvent>,
        before: Option<AuditLogEntryId>,
        limit: Option<u8>,
    },
    GetBan {
        guild_id: GuildId,
        user_id: UserId,
    },
    GetBans {
        guild_id: GuildId,
    },
    GetBotGateway,
    GetChannel {
        channel_id: ChannelId,
    },
    GetChannels {
        guild_id: GuildId,
    },
    GetChannelWebhooks {
        channel_id: ChannelId,
    },
    GetCurrentUser,
    GetCurrentUserGuilds,
    GetEmoji {
        guild_id: GuildId,
        emoji_id: EmojiId,
    },
    GetGateway,
    GetGuild {
        guild_id: GuildId,
    },
    GetGuildEmbed {
        guild_id: GuildId,
    },
    GetGuildEmojis {
        guild_id: GuildId,
    },
    GetGuildIntegrations {
        guild_id: GuildId,
    },
    GetGuildInvites {
        guild_id: GuildId,
    },
    GetGuildMembers {
        guild_id: GuildId,
        limit: Option<u16>,
        after: Option<UserId>,
    },
    GetGuildPruneCount {
        guild_id: GuildId,
        days: Option<u64>,
    },
    GetGuildRegions {
        guild_id: GuildId,
    },
    GetGuildVanityUrl {
        guild_id: GuildId,
    },
    GetGuildWebhooks {
        guild_id: GuildId,
    },
    GetInvite {
        code: &'a str,
        with_counts: Option<bool>,
    },
    GetInvites {
        channel_id: ChannelId,
    },
    GetMember {
        guild_id: GuildId,
        user_id: UserId,
    },
    GetMessage {
        channel_id: ChannelId,
        message_id: MessageId,
    },
    GetMessages {
        channel_id: ChannelId,
        // The keys (before, after, and around) are mutually exclusive.
        // Use an enum to simulate this behaviour.
        around: Option<AroundMessage>,
        limit: Option<u8>,
    },
    GetPins {
        channel_id: ChannelId,
    },
    GetReactionUsers {
        channel_id: ChannelId,
        message_id: MessageId,
        emoji: Emoji,
        before: Option<UserId>,
        after: Option<UserId>,
        limit: Option<u8>,
    },
    GetRoles {
        guild_id: GuildId,
    },
    GetUser {
        user_id: UserId,
    },
    GetVoiceRegions,
    GetWebhook {
        webhook_id: WebhookId,
    },
    GetWebhookWithToken {
        webhook_id: WebhookId,
        token: &'a str,
    },
    KickMember {
        guild_id: GuildId,
        user_id: UserId,
    },
    LeaveGuild {
        guild_id: GuildId,
    },
    PinMessage {
        channel_id: ChannelId,
        message_id: MessageId,
    },
    PruneGuildMembers {
        guild_id: GuildId,
        days: Option<u64>,
        compute_prune_count: Option<bool>,
    },
    RemoveGroupRecipient {
        channel_id: ChannelId,
        user_id: UserId,
    },
    RemoveMemberRole {
        guild_id: GuildId,
        user_id: UserId,
        role_id: RoleId,
    },
    SyncIntegration {
        guild_id: GuildId,
        integration_id: IntegrationId,
    },
    UnbanMember {
        guild_id: GuildId,
        user_id: UserId,
    },
    UnpinMessage {
        channel_id: ChannelId,
        message_id: MessageId,
    },
}

impl<'a> Route<'a> {
    #[remain::check]
    pub(crate) fn method(&self) -> Method {
        use self::Route::*;

        #[remain::sorted]
        match self {
            AddGroupRecipient { .. } => Method::Put,
            AddMemberRole { .. } => Method::Put,
            BanMember { .. } => Method::Put,
            BroadcastTyping { .. } => Method::Post,
            CreateChannel { .. } => Method::Post,
            CreateChannelWebhook { .. } => Method::Post,
            CreateEmoji { .. } => Method::Post,
            CreateGuild => Method::Post,
            CreateIntegration { .. } => Method::Post,
            CreateInvite { .. } => Method::Post,
            CreateMessage { .. } => Method::Post,
            CreatePrivateChannel => Method::Post,
            CreateReaction { .. } => Method::Put,
            CreateRole { .. } => Method::Post,
            DeleteChannel { .. } => Method::Delete,
            DeleteChannelPermission { .. } => Method::Delete,
            DeleteEmoji { .. } => Method::Delete,
            DeleteGuild { .. } => Method::Delete,
            DeleteIntegration { .. } => Method::Delete,
            DeleteInvite { .. } => Method::Delete,
            DeleteMessage { .. } => Method::Delete,
            DeleteMessagesBulk { .. } => Method::Delete,
            DeleteOwnReaction { .. } => Method::Delete,
            DeleteReaction { .. } => Method::Delete,
            DeleteReactions { .. } => Method::Delete,
            DeleteRole { .. } => Method::Delete,
            DeleteWebhook { .. } => Method::Delete,
            DeleteWebhookWithToken { .. } => Method::Delete,
            EditChannel { .. } => Method::Patch,
            EditChannelPermission { .. } => Method::Put,
            EditChannelPositions { .. } => Method::Patch,
            EditCurrentUser => Method::Patch,
            EditEmoji { .. } => Method::Patch,
            EditGuild { .. } => Method::Patch,
            EditGuildEmbed { .. } => Method::Patch,
            EditIntegration { .. } => Method::Patch,
            EditMember { .. } => Method::Patch,
            EditMessage { .. } => Method::Patch,
            EditNickname { .. } => Method::Patch,
            EditRole { .. } => Method::Patch,
            EditRolePositions { .. } => Method::Patch,
            EditWebhook { .. } => Method::Patch,
            EditWebhookWithToken { .. } => Method::Patch,
            ExecuteWebhook { .. } => Method::Post,
            GetAuditLogs { .. } => Method::Get,
            GetBan { .. } => Method::Get,
            GetBans { .. } => Method::Get,
            GetBotGateway => Method::Get,
            GetChannel { .. } => Method::Get,
            GetChannels { .. } => Method::Get,
            GetChannelWebhooks { .. } => Method::Get,
            GetCurrentUser => Method::Get,
            GetCurrentUserGuilds => Method::Get,
            GetEmoji { .. } => Method::Get,
            GetGateway => Method::Get,
            GetGuild { .. } => Method::Get,
            GetGuildEmbed { .. } => Method::Get,
            GetGuildEmojis { .. } => Method::Get,
            GetGuildIntegrations { .. } => Method::Get,
            GetGuildInvites { .. } => Method::Get,
            GetGuildMembers { .. } => Method::Get,
            GetGuildPruneCount { .. } => Method::Get,
            GetGuildRegions { .. } => Method::Get,
            GetGuildVanityUrl { .. } => Method::Get,
            GetGuildWebhooks { .. } => Method::Get,
            GetInvite { .. } => Method::Get,
            GetInvites { .. } => Method::Get,
            GetMember { .. } => Method::Get,
            GetMessage { .. } => Method::Get,
            GetMessages { .. } => Method::Get,
            GetPins { .. } => Method::Get,
            GetReactionUsers { .. } => Method::Get,
            GetRoles { .. } => Method::Get,
            GetUser { .. } => Method::Get,
            GetVoiceRegions => Method::Get,
            GetWebhook { .. } => Method::Get,
            GetWebhookWithToken { .. } => Method::Get,
            KickMember { .. } => Method::Delete,
            LeaveGuild { .. } => Method::Delete,
            PinMessage { .. } => Method::Put,
            PruneGuildMembers { .. } => Method::Post,
            RemoveGroupRecipient { .. } => Method::Delete,
            RemoveMemberRole { .. } => Method::Delete,
            SyncIntegration { .. } => Method::Post,
            UnbanMember { .. } => Method::Delete,
            UnpinMessage { .. } => Method::Delete,
        }
    }

    pub(crate) fn bucket(&self) -> Bucket {
        use self::Route::*;

        match *self {
            GetAuditLogs { guild_id, .. } => Bucket::GuildsIdAuditLogs(guild_id),

            GetChannel { channel_id }
            | EditChannel { channel_id }
            | DeleteChannel { channel_id } => Bucket::ChannelsId(channel_id),

            GetMessages { channel_id, .. } | CreateMessage { channel_id } => {
                Bucket::ChannelsIdMessages(channel_id)
            }

            GetMessage { channel_id, .. } => Bucket::ChannelsIdMessagesId(channel_id),

            CreateReaction { channel_id, .. }
            | DeleteReaction { channel_id, .. }
            | DeleteOwnReaction { channel_id, .. } => {
                Bucket::ChannelsIdMessagesIdReactionsEmojiUserId(channel_id)
            }

            GetReactionUsers { channel_id, .. } => {
                Bucket::ChannelsIdMessagesIdReactionsEmoji(channel_id)
            }

            DeleteReactions { channel_id, .. } => Bucket::ChannelsIdMessagesIdReactions(channel_id),

            EditMessage { channel_id, .. } => Bucket::ChannelsIdMessagesId(channel_id),
            DeleteMessage { channel_id, .. } => Bucket::ChannelsIdMessagesIdDelete(channel_id),

            DeleteMessagesBulk { channel_id, .. } => {
                Bucket::ChannelsIdMessagesBulkDelete(channel_id)
            }

            EditChannelPermission { channel_id, .. }
            | DeleteChannelPermission { channel_id, .. } => {
                Bucket::ChannelsIdPermissionsOverwriteId(channel_id)
            }

            GetInvites { channel_id } | CreateInvite { channel_id } => {
                Bucket::ChannelsIdInvites(channel_id)
            }

            BroadcastTyping { channel_id } => Bucket::ChannelsIdTyping(channel_id),

            GetPins { channel_id } => Bucket::ChannelsIdPins(channel_id),

            PinMessage { channel_id, .. } | UnpinMessage { channel_id, .. } => {
                Bucket::ChannelsIdPinsMessageId(channel_id)
            }

            AddGroupRecipient { channel_id, .. } | RemoveGroupRecipient { channel_id, .. } => {
                Bucket::ChannelsIdRecipientsUserId(channel_id)
            }

            GetGuildEmojis { guild_id } | CreateEmoji { guild_id } => {
                Bucket::GuildsIdEmojis(guild_id)
            }

            GetEmoji { guild_id, .. }
            | EditEmoji { guild_id, .. }
            | DeleteEmoji { guild_id, .. } => Bucket::GuildsIdEmojisId(guild_id),

            CreateGuild => Bucket::Guilds,

            GetGuild { guild_id } | EditGuild { guild_id } | DeleteGuild { guild_id } => {
                Bucket::GuildsId(guild_id)
            }

            GetChannels { guild_id }
            | CreateChannel { guild_id }
            | EditChannelPositions { guild_id } => Bucket::GuildsIdChannels(guild_id),

            GetMember { guild_id, .. }
            | EditMember { guild_id, .. }
            | KickMember { guild_id, .. } => Bucket::GuildsIdMembersId(guild_id),

            GetGuildMembers { guild_id, .. } => Bucket::GuildsIdMembers(guild_id),

            EditNickname { guild_id, .. } => Bucket::GuildsIdMembersMeNick(guild_id),

            AddMemberRole { guild_id, .. } | RemoveMemberRole { guild_id, .. } => {
                Bucket::GuildsIdMembersIdRolesId(guild_id)
            }

            GetBans { guild_id } => Bucket::GuildsIdBans(guild_id),

            GetBan { guild_id, .. } | BanMember { guild_id, .. } | UnbanMember { guild_id, .. } => {
                Bucket::GuildsIdBansUserId(guild_id)
            }

            GetRoles { guild_id } | CreateRole { guild_id } | EditRolePositions { guild_id } => {
                Bucket::GuildsIdRoles(guild_id)
            }

            EditRole { guild_id, .. } | DeleteRole { guild_id, .. } => {
                Bucket::GuildsIdRolesId(guild_id)
            }

            GetGuildPruneCount { guild_id, .. } | PruneGuildMembers { guild_id, .. } => {
                Bucket::GuildsIdPrune(guild_id)
            }

            GetGuildRegions { guild_id } => Bucket::GuildsIdRegions(guild_id),

            GetGuildInvites { guild_id } => Bucket::GuildsIdInvites(guild_id),

            GetGuildIntegrations { guild_id } | CreateIntegration { guild_id } => {
                Bucket::GuildsIdIntegrations(guild_id)
            }

            EditIntegration { guild_id, .. } | DeleteIntegration { guild_id, .. } => {
                Bucket::GuildsIdIntegrationsId(guild_id)
            }

            SyncIntegration { guild_id, .. } => Bucket::GuildsIdIntegrationsIdSync(guild_id),

            GetGuildEmbed { guild_id } | EditGuildEmbed { guild_id } => {
                Bucket::GuildsIdEmbed(guild_id)
            }

            GetGuildVanityUrl { guild_id } => Bucket::GuildsIdVanityUrl(guild_id),

            GetInvite { .. } | DeleteInvite { .. } => Bucket::InvitesCode,

            GetCurrentUser | EditCurrentUser | GetUser { .. } => Bucket::UsersId,

            GetCurrentUserGuilds => Bucket::UsersMeGuilds,

            LeaveGuild { guild_id } => Bucket::UsersMeGuildsId(guild_id),

            CreatePrivateChannel => Bucket::UsersMeChannels,

            GetVoiceRegions => Bucket::VoiceRegions,

            CreateChannelWebhook { channel_id } | GetChannelWebhooks { channel_id } => {
                Bucket::ChannelsIdWebhooks(channel_id)
            }

            GetGuildWebhooks { guild_id } => Bucket::GuildsIdWebhooks(guild_id),

            GetWebhook { webhook_id }
            | EditWebhook { webhook_id }
            | DeleteWebhook { webhook_id } => Bucket::WebhooksId(webhook_id),

            GetWebhookWithToken { webhook_id, .. }
            | EditWebhookWithToken { webhook_id, .. }
            | DeleteWebhookWithToken { webhook_id, .. }
            | ExecuteWebhook { webhook_id, .. } => Bucket::WebhooksIdToken(webhook_id),

            GetGateway => Bucket::Gateway,
            GetBotGateway => Bucket::GatewayBot,
        }
    }

    pub(crate) fn url(&self) -> Cow<'a, str> {
        use self::Route::*;

        match self {
            GetAuditLogs {
                guild_id,
                user_id,
                action_type,
                before,
                limit,
            } => {
                let action_type = action_type.map(u8::from);
                Cow::from(api!("/guilds/{}/audit-logs", guild_id; [
                    ("user_id", user_id?),
                    ("action_type", action_type?),
                    ("before", before?),
                    ("limit", limit?),
                ]))
            }

            GetChannel { channel_id }
            | EditChannel { channel_id }
            | DeleteChannel { channel_id } => Cow::from(api!("/channels/{}", channel_id)),

            GetMessages {
                channel_id,
                around,
                limit,
            } => {
                let mut s = api!("/channels/{}/messages", channel_id; [
                    ("limit", limit?),
                ]);
                let _ = match around {
                    Some(AroundMessage::Around(message_id)) => write!(s, "&around={}", message_id),
                    Some(AroundMessage::Before(message_id)) => write!(s, "&before={}", message_id),
                    Some(AroundMessage::After(message_id)) => write!(s, "&after={}", message_id),
                    None => Ok(()),
                };
                Cow::from(s)
            }

            GetMessage {
                channel_id,
                message_id,
            } => Cow::from(api!("/channels/{}/messages/{}", channel_id, message_id)),

            CreateMessage { channel_id } => Cow::from(api!("/channels/{}/messages", channel_id)),

            CreateReaction {
                channel_id,
                message_id,
                emoji,
            }
            | DeleteOwnReaction {
                channel_id,
                message_id,
                emoji,
            } => Cow::from(api!(
                "/channels/{}/messages/{}/reactions/{}/@me",
                channel_id,
                message_id,
                emoji,
            )),

            DeleteReaction {
                channel_id,
                message_id,
                emoji,
                user_id,
            } => Cow::from(api!(
                "/channels/{}/messages/{}/reactions/{}/{}",
                channel_id,
                message_id,
                emoji,
                user_id,
            )),

            GetReactionUsers {
                channel_id,
                message_id,
                emoji,
                before,
                after,
                limit,
            } => Cow::from(api!(
                "/channels/{}/messages/{}/reactions/{}",
                channel_id,
                message_id,
                emoji;
                [
                    ("before", before?),
                    ("after", after?),
                    ("limit", limit?),
                ]
            )),

            DeleteReactions {
                channel_id,
                message_id,
            } => Cow::from(api!(
                "/channels/{}/messages/{}/reactions",
                channel_id,
                message_id,
            )),

            EditMessage {
                channel_id,
                message_id,
            }
            | DeleteMessage {
                channel_id,
                message_id,
            } => Cow::from(api!("/channels/{}/messages/{}", channel_id, message_id)),

            DeleteMessagesBulk { channel_id } => {
                Cow::from(api!("/channels/{}/messages/bulk-delete", channel_id))
            }

            EditChannelPermission {
                channel_id,
                overwrite_id,
            }
            | DeleteChannelPermission {
                channel_id,
                overwrite_id,
            } => Cow::from(api!(
                "/channels/{}/permissions/{}",
                channel_id,
                overwrite_id,
            )),

            GetInvites { channel_id } | CreateInvite { channel_id } => {
                Cow::from(api!("/channels/{}/invites", channel_id))
            }

            BroadcastTyping { channel_id } => Cow::from(api!("/channels/{}/typing", channel_id)),

            GetPins { channel_id } => Cow::from(api!("/channels/{}/pins", channel_id)),

            PinMessage {
                channel_id,
                message_id,
            }
            | UnpinMessage {
                channel_id,
                message_id,
            } => Cow::from(api!("/channels/{}/pins/{}", channel_id, message_id)),

            AddGroupRecipient {
                channel_id,
                user_id,
            }
            | RemoveGroupRecipient {
                channel_id,
                user_id,
            } => Cow::from(api!("/channels/{}/recipients/{}", channel_id, user_id)),

            GetGuildEmojis { guild_id } | CreateEmoji { guild_id } => {
                Cow::from(api!("/guilds/{}/emojis", guild_id))
            }

            GetEmoji { guild_id, emoji_id }
            | EditEmoji { guild_id, emoji_id }
            | DeleteEmoji { guild_id, emoji_id } => {
                Cow::from(api!("/guilds/{}/emojis/{}", guild_id, emoji_id))
            }

            CreateGuild => Cow::from(api!("/guilds")),

            GetGuild { guild_id } | EditGuild { guild_id } | DeleteGuild { guild_id } => {
                Cow::from(api!("/guilds/{}", guild_id))
            }

            GetChannels { guild_id }
            | CreateChannel { guild_id }
            | EditChannelPositions { guild_id } => Cow::from(api!("/guilds/{}/channels", guild_id)),

            GetMember { guild_id, user_id }
            | EditMember { guild_id, user_id }
            | KickMember { guild_id, user_id } => {
                Cow::from(api!("/guilds/{}/members/{}", guild_id, user_id))
            }

            GetGuildMembers {
                guild_id,
                limit,
                after,
            } => Cow::from(api!("/guilds/{}/members", guild_id; [
                ("limit", limit?),
                ("after", after?),
            ])),

            EditNickname { guild_id } => Cow::from(api!("/guilds/{}/members/@me/nick", guild_id)),

            AddMemberRole {
                guild_id,
                user_id,
                role_id,
            }
            | RemoveMemberRole {
                guild_id,
                user_id,
                role_id,
            } => Cow::from(api!(
                "/guilds/{}/members/{}/roles/{}",
                guild_id,
                user_id,
                role_id,
            )),

            GetBans { guild_id } => Cow::from(api!("/guilds/{}/bans", guild_id)),

            GetBan { guild_id, user_id } | UnbanMember { guild_id, user_id } => {
                Cow::from(api!("/guilds/{}/bans/{}", guild_id, user_id))
            }

            BanMember {
                guild_id,
                user_id,
                delete_message_days,
                reason,
            } => Cow::from(api!("/guilds/{}/bans/{}", guild_id, user_id; [
                ("delete-message-days", delete_message_days?),
                ("reason", reason?),
            ])),

            GetRoles { guild_id } | CreateRole { guild_id } | EditRolePositions { guild_id } => {
                Cow::from(api!("/guilds/{}/roles", guild_id))
            }

            EditRole { guild_id, role_id } | DeleteRole { guild_id, role_id } => {
                Cow::from(api!("/guilds/{}/roles/{}", guild_id, role_id))
            }

            GetGuildPruneCount { guild_id, days } => {
                Cow::from(api!("/guilds/{}/prune", guild_id; [
                    ("days", days?),
                ]))
            }

            PruneGuildMembers {
                guild_id,
                days,
                compute_prune_count,
            } => Cow::from(api!("/guilds/{}/prune", guild_id; [
                ("days", days?),
                ("compute_prune_count", compute_prune_count?),
            ])),

            GetGuildRegions { guild_id } => Cow::from(api!("/guilds/{}/regions", guild_id)),

            GetGuildInvites { guild_id } => Cow::from(api!("/guilds/{}/invites", guild_id)),

            GetGuildIntegrations { guild_id } | CreateIntegration { guild_id } => {
                Cow::from(api!("/guilds/{}/integrations", guild_id))
            }

            EditIntegration {
                guild_id,
                integration_id,
            }
            | DeleteIntegration {
                guild_id,
                integration_id,
            } => Cow::from(api!("/guilds/{}/integrations/{}", guild_id, integration_id)),

            SyncIntegration {
                guild_id,
                integration_id,
            } => Cow::from(api!(
                "/guilds/{}/integrations/{}/sync",
                guild_id,
                integration_id,
            )),

            GetGuildEmbed { guild_id } | EditGuildEmbed { guild_id } => {
                Cow::from(api!("/guilds/{}/embed", guild_id))
            }

            GetGuildVanityUrl { guild_id } => Cow::from(api!("/guilds/{}/vanity-url", guild_id)),

            GetInvite { code, with_counts } => Cow::from(api!("/invites/{}", code; [
                ("with_counts", with_counts?),
            ])),

            DeleteInvite { code } => Cow::from(api!("/invites/{}", code)),

            GetCurrentUser | EditCurrentUser => Cow::from(api!("/users/@me")),

            GetCurrentUserGuilds => Cow::from(api!("/users/@me/guilds")),

            GetUser { user_id } => Cow::from(api!("/users/{}", user_id)),

            LeaveGuild { guild_id } => Cow::from(api!("/user/@me/guilds/{}", guild_id)),

            CreatePrivateChannel => Cow::from(api!("/users/@me/channels")),

            GetVoiceRegions => Cow::from(api!("/voice/regions")),

            CreateChannelWebhook { channel_id } | GetChannelWebhooks { channel_id } => {
                Cow::from(api!("/channels/{}/webhooks", channel_id))
            }

            GetGuildWebhooks { guild_id } => Cow::from(api!("/guilds/{}/webhooks", guild_id)),

            GetWebhook { webhook_id }
            | EditWebhook { webhook_id }
            | DeleteWebhook { webhook_id } => Cow::from(api!("/webhooks/{}", webhook_id)),

            GetWebhookWithToken { webhook_id, token }
            | EditWebhookWithToken { webhook_id, token }
            | DeleteWebhookWithToken { webhook_id, token } => {
                Cow::from(api!("/webhooks/{}/{}", webhook_id, token))
            }

            ExecuteWebhook {
                webhook_id,
                token,
                wait,
            } => Cow::from(api!("/webhooks/{}/{}", webhook_id, token; [
                ("wait", wait?),
            ])),

            GetGateway => Cow::from(api!("/gateway")),
            GetBotGateway => Cow::from(api!("/gateway/bot")),
        }
    }
}

/// Methods implementing `Copy`, with mappings to corresponding reqwest methods.
#[remain::sorted]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Method {
    Delete,
    Get,
    Patch,
    Post,
    Put,
}

impl From<Method> for HttpMethod {
    fn from(method: Method) -> Self {
        match method {
            Method::Delete => HttpMethod::DELETE,
            Method::Get => HttpMethod::GET,
            Method::Patch => HttpMethod::PATCH,
            Method::Post => HttpMethod::POST,
            Method::Put => HttpMethod::PUT,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum AroundMessage {
    Around(MessageId),
    Before(MessageId),
    After(MessageId),
}
