use async_std::sync::Arc;

use bytes::Bytes;
use hyper::StatusCode;
use serde::de::DeserializeOwned;
use serde::{Serialize, Serializer};

use crate::builder::marker::GuildChannelBuilder;
use crate::builder::{
    CreateChannel, CreateGuild, CreateInvite, CreateMessage, CreateRole, EditChannel,
};
use crate::internal::prelude::*;
use crate::model::channel::permissions::{OverwriteId, PermissionOverwrite};
use crate::model::channel::{Channel, DMChannel, Message};
use crate::model::emoji::{Emoji, PartialEmoji};
use crate::model::guild::invite::Invite;
use crate::model::guild::{Guild, Role};
use crate::model::id::{ChannelId, EmojiId, GuildId, MessageId, RoleId, UserId, WebhookId};
use crate::model::voice::VoiceRegionId;
use crate::model::webhook::Webhook;

use super::error::ErrorResponse;
use super::prelude::*;
use super::ratelimit::RateLimiter;

/// An HTTP client for performing requests to the REST API.
pub struct Http {
    /// Internal rate limit manager.
    ratelimiter: RateLimiter,
}

impl Http {
    /// Creates a new HTTP client with the given API token.
    pub fn new<S: AsRef<str>>(token: S) -> Http {
        // Trim whitespace from token.
        let token = token.as_ref().trim();
        // Add "Bot " prefix to token if necessary.
        let token = if token.starts_with("Bot ") {
            Bytes::copy_from_slice(token.as_bytes())
        } else {
            Bytes::from(format!("Bot {}", token))
        };

        let client = hyper::Client::builder().build(HttpsConnector::new());
        let client = Arc::new(client);

        Http {
            ratelimiter: RateLimiter::new(client, token),
        }
    }

    /// Adds a [`User`] to a [`Group`].
    ///
    /// This requires an access token of the user, granted to an app by the
    /// `gdm.join` scope.
    ///
    /// [`User`]: ../model/user/struct.User.html
    /// [`Group`]: ../model/channel/struct.Group.html
    pub async fn add_group_recipient(&self, channel_id: ChannelId, user_id: UserId) -> Result<()> {
        self.fire(Request::new(Route::AddGroupRecipient {
            channel_id,
            user_id,
        }))
        .await
    }

    /// Adds a [`Role`] to a guild [`Member`].
    ///
    /// Requires the [`MANAGE_ROLES`] permission.
    ///
    /// [`Role`]: ../model/guild/struct.Role.html
    /// [`Member`]: ../model/guild/struct.Member.html
    #[doc = "\n[`MANAGE_ROLES`]: ../model/permissions/struct.Permissions.html#associatedconstant.MANAGE_ROLES"]
    pub async fn add_member_role(
        &self,
        guild_id: GuildId,
        user_id: UserId,
        role_id: RoleId,
    ) -> Result<()> {
        self.fire(Request::new(Route::AddMemberRole {
            guild_id,
            user_id,
            role_id,
        }))
        .await
    }

    /// Bans a [`Member`] from the [`Guild`] and optionally deletes messages
    /// send by the user in the last `delete_message_days` days.
    ///
    /// Requires the [`BAN_MEMBERS`] permission.
    ///
    /// # Notes
    ///
    /// The maximum number of days that messages can be deleted for is 7 days.
    /// If `delete_message_days > 7`, it will be truncated to 7 days.
    ///
    /// [`Member`]: ../model/guild/struct.Member.html
    /// [`Guild`]: ../model/guild/struct.Guild.html
    #[doc = "\n[`BAN_MEMBERS`]: ../model/permissions/struct.Permissions.html#associatedconstant.BAN_MEMBERS"]
    pub async fn ban_member(
        &self,
        guild_id: GuildId,
        user_id: UserId,
        delete_message_days: Option<u8>,
        reason: Option<&str>,
    ) -> Result<()> {
        let delete_message_days = match delete_message_days {
            days @ Some(0..=7) | days @ None => days,
            Some(days) => {
                log::warn!(
                    "messages can only be deleted for the last 7 days: {} will be limited to 7",
                    days
                );
                Some(7)
            }
        };

        self.fire(Request::new(Route::BanMember {
            guild_id,
            user_id,
            delete_message_days,
            reason,
        }))
        .await
    }

    /// Posts a typing indicator for the specified [`Channel`].
    ///
    /// # Notes
    ///
    /// Generally bots should not implement this route. However, if a bot is
    /// responding to a command and expects the computation to take a few
    /// seconds, this endpoint may be called to let the user know that the bot
    /// is processing their message.
    ///
    /// [`Channel`]: ../model/channel/enum.Channel.html
    pub async fn broadcast_typing(&self, channel_id: ChannelId) -> Result<()> {
        self.fire(Request::new(Route::BroadcastTyping { channel_id }))
            .await
    }

    /// Creates a new [`GuildChannel`] in the specified [`Guild`].
    ///
    /// Requires the [`MANAGE_CHANNELS`] permission.
    ///
    /// [`GuildChannel`]: ../model/channel/guild/enum.GuildChannel.html
    /// [`Guild`]: ../model/guild/struct.Guild.html
    #[doc = "\n[`MANAGE_CHANNELS`]: ../model/permissions/struct.Permissions.html#associatedconstant.MANAGE_CHANNELS"]
    pub async fn create_channel<T, F>(
        &self,
        guild_id: GuildId,
        name: &str,
        create_channel: F,
    ) -> Result<T>
    where
        T: GuildChannelBuilder + DeserializeOwned,
        F: FnOnce(&mut CreateChannel<T>),
    {
        let mut channel = CreateChannel::<T>::create(name);
        create_channel(&mut channel);

        let mut request = Request::new(Route::CreateChannel { guild_id });
        request.json(&channel)?;

        self.request(request).await
    }

    /// Creates a new [`Webhook`] for the specified [`GuildChannel`].
    ///
    /// Requires the [`MANAGE_CHANNELS`] permission.
    ///
    /// [`Webhook`]: ../model/webhook/struct.Webhook.html
    /// [`GuildChannel`]: ../model/channel/guild/enum.GuildChannel.html
    #[doc = "\n[`MANAGE_CHANNELS`]: ../model/permissions/struct.Permissions.html#associatedconstant.MANAGE_CHANNELS"]
    pub async fn create_webhook(
        &self,
        channel_id: ChannelId,
        name: &str,
        avatar: Option<&str>,
    ) -> Result<Webhook> {
        #[derive(Debug, Serialize)]
        struct Params<'a> {
            name: &'a str,
            avatar: Option<&'a str>,
        }
        let params = Params { name, avatar };

        let mut request = Request::new(Route::CreateChannelWebhook { channel_id });
        request.json(&params)?;

        self.request(request).await
    }

    /// Creates a new [`Emoji`] in the specified [`Guild`].
    ///
    /// Requires the [`MANAGE_EMOJIS`] permission.
    ///
    /// # Notes
    ///
    /// The `image` must be a base64 encoded image in the form of a
    /// [Data URI scheme], supported image formats are JPG, GIF and PNG.
    ///
    /// An example Data URI format is:
    /// ```text
    /// data:image/jpeg;base64,BASE64_ENCODED_JPEG_IMAGE_DATA
    /// ```
    ///
    /// [`Emoji`]: ../model/guild/struct.Emoji.html
    /// [`Guild`]: ../model/guild/struct.Guild.html
    /// [Data URI scheme]: https://en.wikipedia.org/wiki/Data_URI_scheme
    #[doc = "\n[`MANAGE_EMOJIS`]: ../model/permissions/struct.Permissions.html#associatedconstant.MANAGE_EMOJIS"]
    pub async fn create_emoji(
        &self,
        guild_id: GuildId,
        name: &str,
        image: &str,
        roles: &[RoleId],
    ) -> Result<Emoji> {
        #[derive(Debug, Serialize)]
        struct Params<'a> {
            name: &'a str,
            image: &'a str,
            roles: &'a [RoleId],
        }
        let params = Params { name, image, roles };

        let mut request = Request::new(Route::CreateEmoji { guild_id });
        request.json(&params)?;

        self.request(request).await
    }

    /// Creates a new [`Guild`].
    ///
    /// This can be used only by bots in fewer than 10 guilds.
    ///
    /// [`Guild`]: ../model/guild/struct.Guild.html
    pub async fn create_guild<F>(
        &self,
        name: &str,
        region: VoiceRegionId,
        create_guild: F,
    ) -> Result<Guild>
    where
        F: FnOnce(&mut CreateGuild),
    {
        let mut guild = CreateGuild::create(name, region);
        create_guild(&mut guild);

        let mut request = Request::new(Route::CreateGuild);
        request.json(&guild)?;

        self.request(request).await
    }

    // TODO: Add create_integration.

    /// Creates a new [`Invite`] for the specified [`GuildChannel`].
    ///
    /// Requires the [`CREATE_INSTANT_INVITE`] permission.
    ///
    /// [`Invite`]: ../model/guild/invite/struct.Invite.html
    /// [`GuildChannel`]: ../model/channel/guild/enum.GuildChannel.html
    #[doc = "\n[`CREATE_INSTANT_INVITE`]: ../model/permissions/struct.Permissions.html#associatedconstant.CREATE_INSTANT_INVITE"]
    pub async fn create_invite<F>(&self, channel_id: ChannelId, create_invite: F) -> Result<Invite>
    where
        F: FnOnce(&mut CreateInvite),
    {
        let mut invite = CreateInvite::create();
        create_invite(&mut invite);

        let mut request = Request::new(Route::CreateInvite { channel_id });
        request.json(&invite)?;

        self.request(request).await
    }

    /// Creates a new [`Message`] in the specified [`Channel`].
    ///
    /// Requires the [`SEND_MESSAGES`] permission if in a [`Guild`].
    ///
    /// [`Message`]: ../model/channel/message/struct.Message.html
    /// [`Channel`]: ../model/channel/enum.Channel.html
    /// [`Guild`]: ../model/guild/struct.Guild.html
    #[doc = "\n[`SEND_MESSAGES`]: ../model/permissions/struct.Permissions.html#associatedconstant.SEND_MESSAGES"]
    pub async fn create_message<F>(
        &self,
        channel_id: ChannelId,
        create_message: F,
    ) -> Result<Message>
    where
        F: FnOnce(&mut CreateMessage),
    {
        let mut msg = CreateMessage::create();
        create_message(&mut msg);

        let mut request = Request::new(Route::CreateMessage { channel_id });
        request.json(&msg)?;

        self.request(request).await
    }

    /// Creates a new [`DMChannel`] with the specified recipient user.
    ///
    /// [`DMChannel`]: ../model/channel/struct.DMChannel.html
    pub async fn create_dm(&self, recipient_id: UserId) -> Result<DMChannel> {
        #[derive(Debug, Serialize)]
        struct Params {
            recipient_id: UserId,
        }
        let params = Params { recipient_id };

        let mut request = Request::new(Route::CreatePrivateChannel);
        request.json(&params)?;

        self.request(request).await
    }

    /// Creates a reaction on the specified message.
    ///
    /// Requires the [`READ_MESSAGE_HISTORY`] permission for the channel.
    ///
    /// Additionally requires the [`ADD_REACTIONS`] permission for the channel,
    /// if nobody else has reacted to the message using this emoji.
    #[doc = "\n[`READ_MESSAGE_HISTORY`]: ../model/permissions/struct.Permissions.html#associatedconstant.READ_MESSAGE_HISTORY"]
    #[doc = "\n[`ADD_REACTIONS`]: ../model/permissions/struct.Permissions.html#associatedconstant.ADD_REACTIONS"]
    pub async fn create_reaction(
        &self,
        channel_id: ChannelId,
        message_id: MessageId,
        emoji: PartialEmoji,
    ) -> Result<()> {
        self.fire(Request::new(Route::CreateReaction {
            channel_id,
            message_id,
            emoji,
        }))
        .await
    }

    /// Creates a new [`Role`] for the specified guild.
    ///
    /// Requires the [`MANAGE_ROLES`] permission.
    ///
    /// [`Role`]: ../model/guild/struct.Role.html
    #[doc = "\n[`MANAGE_ROLES`]: ../model/permissions/struct.Permissions.html#associatedconstant.MANAGE_ROLES"]
    pub async fn create_role<F>(&self, guild_id: GuildId, create_role: F) -> Result<Role>
    where
        F: FnOnce(&mut CreateRole),
    {
        let mut role = CreateRole::create();
        create_role(&mut role);

        let mut request = Request::new(Route::CreateRole { guild_id });
        request.json(&role)?;

        self.request(request).await
    }

    /// Deletes a [`GuildChannel`] or closes a [`DMChannel`].
    ///
    /// Requires the [`MANAGE_CHANNELS`] permission to delete a guild channel.
    ///
    /// # Notes
    ///
    /// Deleting a [`Category`] does not delete its child channels.
    ///
    /// # Warnings
    ///
    /// Deleting a guild channel cannot be undone. Use this with caution, as it
    /// is impossible to undo this action when performed on a guild channel. In
    /// contrast, when used with a private message, it is possible to undo the
    /// action by opening a private message with the recipient again.
    ///
    /// [`GuildChannel`]: ../model/channel/guild/enum.GuildChannel.html
    /// [`DMChannel`]: ../model/channel/struct.DMChannel.html
    /// [`Category`]: ../model/channel/guild/struct.Category.html
    #[doc = "\n[`MANAGE_CHANNELS`]: ../model/permissions/struct.Permissions.html#associatedconstant.MANAGE_CHANNELS"]
    pub async fn delete_channel(&self, channel_id: ChannelId) -> Result<Channel> {
        self.request(Request::new(Route::DeleteChannel { channel_id }))
            .await
    }

    /// Deletes a channel permission overwrite for a [`User`] or [`Role`] in the
    /// specified [`GuildChannel`].
    ///
    /// Requires the [`MANAGE_ROLES`] permission.
    ///
    /// [`User`]: ../model/user/struct.User.html
    /// [`Role`]: ../model/guild/struct.Role.html
    /// [`GuildChannel`]: ../model/channel/guild/enum.GuildChannel.html
    #[doc = "\n[`MANAGE_ROLES`]: ../model/permissions/struct.Permissions.html#associatedconstant.MANAGE_ROLES"]
    pub async fn delete_channel_permission(
        &self,
        channel_id: ChannelId,
        overwrite_id: OverwriteId,
    ) -> Result<()> {
        self.fire(Request::new(Route::DeleteChannelPermission {
            channel_id,
            overwrite_id,
        }))
        .await
    }

    /// Deletes an [`Emoji`].
    ///
    /// Requires the [`MANAGE_EMOJIS`] permission.
    ///
    /// [`Emoji`]: ../model/emoji/struct.Emoji.html
    #[doc = "\n[`MANAGE_EMOJIS`]: ../model/permissions/struct.Permissions.html#associatedconstant.MANAGE_EMOJIS"]
    pub async fn delete_emoji(&self, guild_id: GuildId, emoji_id: EmojiId) -> Result<()> {
        self.fire(Request::new(Route::DeleteEmoji { guild_id, emoji_id }))
            .await
    }

    /// Deletes a [`Guild`].
    ///
    /// **Must be the owner of the guild.**
    ///
    /// [`Guild`]: ../model/guild/struct.Guild.html
    pub async fn delete_guild(&self, guild_id: GuildId) -> Result<()> {
        self.fire(Request::new(Route::DeleteGuild { guild_id }))
            .await
    }

    // TODO: Add delete_integration.

    /// Deletes an [`Invite`].
    ///
    /// Requires either the [`MANAGE_CHANNELS`] permission on the channel the
    /// invite belongs to, or the [`MANAGE_GUILD`] permission to delete an
    /// invite across the guild.
    ///
    /// [`Invite`]: ../model/guild/invite/struct.Invite.html
    #[doc = "\n[`MANAGE_CHANNELS`]: ../model/permissions/struct.Permissions.html#associatedconstant.MANAGE_CHANNELS"]
    #[doc = "\n[`MANAGE_GUILD`]: ../model/permissions/struct.Permissions.html#associatedconstant.MANAGE_GUILD"]
    pub async fn delete_invite(&self, invite_code: &str) -> Result<Invite> {
        self.request(Request::new(Route::DeleteInvite { code: invite_code }))
            .await
    }

    /// Deletes a [`Message`].
    ///
    /// Requires [`MANAGE_MESSAGES`] if trying to delete a message not sent by
    /// the client user.
    ///
    /// [`Message`]: ../model/channel/message/struct.Message.html
    #[doc = "\n[`MANAGE_MESSAGE`]: ../model/permissions/struct.Permissions.html#associatedconstant.MANAGE_MESSAGE"]
    pub async fn delete_message(&self, channel_id: ChannelId, message_id: MessageId) -> Result<()> {
        self.fire(Request::new(Route::DeleteMessage {
            channel_id,
            message_id,
        }))
        .await
    }

    /// Deletes multiple [`Message`]s with a single request.
    ///
    /// Requires [`MANAGE_MESSAGES`] if trying to delete a message not sent by
    /// the client user.
    ///
    /// [`Message`]: ../model/channel/message/struct.Message.html
    #[doc = "\n[`MANAGE_MESSAGE`]: ../model/permissions/struct.Permissions.html#associatedconstant.MANAGE_MESSAGE"]
    pub async fn delete_messages_bulk(
        &self,
        channel_id: ChannelId,
        messages: &[MessageId],
    ) -> Result<()> {
        #[derive(Debug, Serialize)]
        struct Params<'a> {
            messages: &'a [MessageId],
        }
        let params = Params { messages };

        let mut request = Request::new(Route::DeleteMessagesBulk { channel_id });
        request.json(&params)?;

        self.fire(Request::new(Route::DeleteMessagesBulk { channel_id }))
            .await
    }

    /// Deletes a [`Reaction`] on the specified [`Message`].
    ///
    /// Requires the [`MANAGE_MESSAGES`] permission if trying to delete a
    /// reaction made by another user (ie. `user_id == Some(other_user_id)`).
    ///
    /// [`Reaction`]: ../model/channel/message/struct.Reaction.html
    /// [`Message`]: ../model/channel/message/struct.Message.html
    #[doc = "\n[`MANAGE_MESSAGES`]: ../model/permissions/struct.Permissions.html#associatedconstant.MANAGE_MESSAGES"]
    pub async fn delete_reaction(
        &self,
        channel_id: ChannelId,
        message_id: MessageId,
        emoji: PartialEmoji,
        user_id: Option<UserId>,
    ) -> Result<()> {
        let route = match user_id {
            Some(user_id) => Route::DeleteReaction {
                channel_id,
                message_id,
                emoji,
                user_id,
            },
            None => Route::DeleteOwnReaction {
                channel_id,
                message_id,
                emoji,
            },
        };
        self.fire(Request::new(route)).await
    }

    /// Deletes all [`Reaction`]s on the specified [`Message`].
    ///
    /// Requires the [`MANAGE_MESSAGES`] permission.
    #[doc = "\n[`MANAGE_MESSAGES`]: ../model/permissions/struct.Permissions.html#associatedconstant.MANAGE_MESSAGES"]
    pub async fn delete_reactions(
        &self,
        channel_id: ChannelId,
        message_id: MessageId,
    ) -> Result<()> {
        self.fire(Request::new(Route::DeleteReactions {
            channel_id,
            message_id,
        }))
        .await
    }

    /// Deletes a [`Role`].
    ///
    /// Requires the [`MANAGE_ROLES`] permission.
    #[doc = "\n[`MANAGE_ROLES`]: ../model/permissions/struct.Permissions.html#associatedconstant.MANAGE_ROLES"]
    pub async fn delete_role(&self, guild_id: GuildId, role_id: RoleId) -> Result<()> {
        self.fire(Request::new(Route::DeleteRole { guild_id, role_id }))
            .await
    }

    /// Deletes a [`Webhook]` permanently.
    ///
    /// **Must be the owner of the guild.**
    pub async fn delete_webhook(&self, webhook_id: WebhookId) -> Result<()> {
        self.fire(Request::new(Route::DeleteWebhook { webhook_id }))
            .await
    }

    /// Edits a [`GuildChannel`].
    ///
    /// Requires the [`MANAGE_CHANNELS`] permission.
    ///
    /// [`GuildChannel`]: ../model/channel/guild/enum.GuildChannel.html
    #[doc = "\n[`MANAGE_CHANNELS`]: ../model/permissions/struct.Permissions.html#associatedconstant.MANAGE_CHANNELS"]
    pub async fn edit_channel<F, T>(&self, channel_id: ChannelId, edit_channel: F) -> Result<T>
    where
        F: FnOnce(&mut EditChannel<T>),
        T: GuildChannelBuilder + DeserializeOwned,
    {
        let mut channel = EditChannel::<T>::new();
        edit_channel(&mut channel);

        let mut request = Request::new(Route::EditChannel { channel_id });
        request.json(&channel)?;

        self.request(request).await
    }

    /// Edits a channel permission overwrite for a [`User`] or [`Role`] in the
    /// specified [`GuildChannel`].
    ///
    /// Requires the [`MANAGE_ROLES`] permission.
    ///
    /// [`User`]: ../model/user/struct.User.html
    /// [`Role`]: ../model/guild/struct.Role.html
    /// [`GuildChannel`]: ../model/channel/guild/enum.GuildChannel.html
    #[doc = "\n[`MANAGE_ROLES`]: ../model/permissions/struct.Permissions.html#associatedconstant.MANAGE_ROLES"]
    pub async fn edit_channel_permission(
        &self,
        channel_id: ChannelId,
        overwrite: PermissionOverwrite,
    ) -> Result<()> {
        let mut request = Request::new(Route::EditChannelPermission {
            channel_id,
            overwrite_id: overwrite.id,
        });
        request.json(&overwrite)?;

        self.fire(request).await
    }

    /// Edits the positions of a set of [`GuildChannel`]s.
    ///
    /// Requires the [`MANAGE_CHANNELS`] permission.
    ///
    /// [`GuildChannel`]: ../model/channel/guild/enum.GuildChannel.html
    #[doc = "\n[`MANAGE_CHANNELS`]: ../model/permissions/struct.Permissions.html#associatedconstant.MANAGE_CHANNELS"]
    pub async fn edit_channel_positions<I>(
        &self,
        guild_id: GuildId,
        channels: &[(ChannelId, usize)],
    ) -> Result<()> {
        #[derive(Debug)]
        struct Params<'a> {
            channels: &'a [(ChannelId, usize)],
        }

        impl<'a> Serialize for Params<'a> {
            fn serialize<S>(&self, serializer: S) -> StdResult<S::Ok, S::Error>
            where
                S: Serializer,
            {
                #[derive(Debug, Serialize)]
                struct Param {
                    id: ChannelId,
                    position: usize,
                }

                use serde::ser::SerializeTuple;

                let mut seq = serializer.serialize_tuple(self.channels.len())?;
                for &(id, position) in self.channels {
                    let param = Param { id, position };
                    seq.serialize_element(&param)?;
                }
                seq.end()
            }
        }

        let params = Params { channels };

        let mut request = Request::new(Route::EditChannelPositions { guild_id });
        request.json(&params)?;

        self.fire(request).await
    }

    /// Performs a request with rate limiting if necessary.
    ///
    /// # Stability
    ///
    /// This is not part of the stable API and may change at any time.
    pub async fn request<T: DeserializeOwned>(&self, request: Request<'_>) -> Result<T> {
        json_body(self.inner_request(request).await?).await
    }

    async fn fire(&self, request: Request<'_>) -> Result<()> {
        let response = self.ratelimiter.perform(&request).await?;

        match response.status() {
            // Expect a `204 - No Content` status code.
            StatusCode::NO_CONTENT => Ok(()),
            // Unexpected status code.
            status => Err(Error::HttpError(HttpError::UnsuccessfulRequest(
                ErrorResponse {
                    status,
                    url: request.route.url().to_string(),
                    error: json_body(response).await.ok(),
                },
            ))),
        }
    }

    async fn inner_request(&self, request: Request<'_>) -> Result<HttpResponse> {
        let response = self.ratelimiter.perform(&request).await?;

        if response.status().is_success() {
            Ok(response)
        } else {
            Err(Error::HttpError(HttpError::UnsuccessfulRequest(
                ErrorResponse {
                    status: response.status(),
                    url: request.route.url().to_string(),
                    error: json_body(response).await.ok(),
                },
            )))
        }
    }
}

async fn json_body<T: DeserializeOwned>(mut response: HttpResponse) -> Result<T> {
    use bytes::buf::BufExt;

    let body = hyper::body::aggregate(response.body_mut())
        .await
        .map_err(HttpError::HyperError)?;
    let result: T = serde_json::from_reader(body.reader())?;
    Ok(result)
}
