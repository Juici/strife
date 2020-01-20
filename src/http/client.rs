use async_std::sync::Arc;

use bytes::Bytes;
use hyper::StatusCode;
use serde::de::DeserializeOwned;

use crate::builder::CreateChannel;
use crate::internal::prelude::*;
use crate::model::id::{ChannelId, GuildId, RoleId, ToSnowflakeId, UserId};

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
    pub async fn add_group_recipient<C, U>(&self, channel: C, user: U) -> Result<()>
    where
        C: ToSnowflakeId<Id = ChannelId>,
        U: ToSnowflakeId<Id = UserId>,
    {
        let channel_id = channel.id();
        let user_id = user.id();

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
    pub async fn add_member_role<G, U, R>(&self, guild: G, user: U, role: R) -> Result<()>
    where
        G: ToSnowflakeId<Id = GuildId>,
        U: ToSnowflakeId<Id = UserId>,
        R: ToSnowflakeId<Id = RoleId>,
    {
        let guild_id = guild.id();
        let user_id = user.id();
        let role_id = role.id();

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
    pub async fn ban_member<G, U>(
        &self,
        guild: G,
        user: U,
        delete_message_days: Option<u8>,
        reason: Option<&str>,
    ) -> Result<()>
    where
        G: ToSnowflakeId<Id = GuildId>,
        U: ToSnowflakeId<Id = UserId>,
    {
        let guild_id = guild.id();
        let user_id = user.id();

        let delete_message_days = match delete_message_days {
            days @ Some(0..=7) => days,
            Some(days) => {
                log::debug!(
                    "messages can only be deleted for the last 7 days: {} will be limited to 7",
                    days
                );
                Some(7)
            }
            None => None,
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
    pub async fn broadcast_typing<C>(&self, channel: C) -> Result<()>
    where
        C: ToSnowflakeId<Id = ChannelId>,
    {
        let channel_id = channel.id();

        self.fire(Request::new(Route::BroadcastTyping { channel_id }))
            .await
    }

    /// Creates a new [`GuildChannel`] in the [`Guild`].
    ///
    /// Requires the [`MANAGE_CHANNELS`] permission.
    ///
    /// [`Channel`]: ../model/channel/enum.Channel.html
    /// [`Guild`]: ../model/guild/struct.Guild.html
    /// [`GuildChannel`]: ../model/channel/guild/enum.GuildChannel.html
    #[doc = "\n[`MANAGE_CHANNELS`]: ../model/permissions/struct.Permissions.html#associatedconstant.MANAGE_CHANNELS"]
    pub async fn create_channel<G, S, F, T>(
        &self,
        guild: G,
        name: S,
        create_channel: F,
    ) -> Result<T>
    where
        G: ToSnowflakeId<Id = GuildId>,
        S: Into<String>,
        F: FnOnce(&mut CreateChannel<T>),
        T: crate::builder::marker::GuildChannelMarker + DeserializeOwned,
    {
        let guild_id = guild.id();

        let mut channel = CreateChannel::<T>::create(name);
        create_channel(&mut channel);

        let mut request = Request::new(Route::CreateChannel { guild_id });
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
