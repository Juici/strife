use async_std::sync::Arc;

use bytes::Bytes;
use hyper::StatusCode;
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::builder::marker::GuildChannelBuilder;
use crate::builder::{CreateChannel, CreateGuild, CreateInvite, EditChannel};
use crate::internal::prelude::*;
use crate::model::emoji::Emoji;
use crate::model::guild::Guild;
use crate::model::id::{ChannelId, GuildId, RoleId, ToSnowflakeId, UserId};
use crate::model::voice::VoiceRegionId;
use crate::model::webhook::Webhook;

use super::error::ErrorResponse;
use super::prelude::*;
use super::ratelimit::RateLimiter;
use crate::model::guild::invite::Invite;

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
    /// [`Emoji`]: ../model/guild/struct.Emoji.html
    /// [`Guild`]: ../model/guild/struct.Guild.html
    #[doc = "\n[`MANAGE_EMOJIS`]: ../model/permissions/struct.Permissions.html#associatedconstant.MANAGE_EMOJIS"]
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
    /// [Data URI scheme]: https://en.wikipedia.org/wiki/Data_URI_scheme
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

    // TODO: create_integration

    /// Create a new [`Invite`] for the specified [`GuildChannel`].
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

    /// Edits a [`GuildChannel`].
    ///
    /// Requires the [`MANAGE_CHANNELS`] permission.
    ///
    /// [`GuildChannel`]: ../model/channel/guild/enum.GuildChannel.html
    #[doc = "\n[`MANAGE_CHANNELS`]: ../model/permissions/struct.Permissions.html#associatedconstant.MANAGE_CHANNELS"]
    pub async fn edit_channel<G, C, F, T>(&self, channel: C, edit_channel: F) -> Result<T>
    where
        C: ToSnowflakeId<Id = ChannelId>,
        F: FnOnce(&mut EditChannel<T>),
        T: GuildChannelBuilder + DeserializeOwned,
    {
        let channel_id = channel.id();

        let mut channel = EditChannel::<T>::new();
        edit_channel(&mut channel);

        let mut request = Request::new(Route::EditChannel { channel_id });
        request.json(&channel)?;

        self.request(request).await
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
