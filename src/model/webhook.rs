//! Models related to webhooks.

use serde::{Deserialize, Serialize};

use crate::model::id::{ChannelId, GuildId, WebhookId};
use crate::model::user::User;

/// A webhook is a low-effort way to post messages to channels in Discord.
///
/// They do not require a bot user or authentication to use.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Webhook {
    /// The ID of the webhook.
    pub id: WebhookId,
    /// The type of the webhook.
    #[serde(rename = "type")]
    pub kind: WebhookType,
    /// The ID of the [`Guild`] containing the [`Channel`] the webhook is for.
    ///
    /// [`Guild`]: ../guild/struct.Guild.html
    /// [`Channel`]: ../channel/enum.Channel.html
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub guild_id: Option<GuildId>,
    /// The ID of the [`Channel`] the webhook is for.
    ///
    /// [`Channel`]: ../channel/enum.Channel.html
    pub channel_id: ChannelId,
    /// The user that created the webhook, this is `None` when getting a webhook
    /// with its [`token`].
    ///
    /// [`token`]: #structfield.token
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user: Option<User>,
    /// The default name of the webhook.
    pub name: Option<String>,
    /// The default avatar of the webhook.
    pub avatar: Option<String>,
    /// The secure token of the webhook (present for [`Incoming`] Webhooks).
    ///
    /// [`Incoming`]: enum.WebhookType.html#variant.Incoming
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
}

/// The type of a [`Webhook`].
///
/// [`Webhook`]: struct.Webhook.html
#[non_exhaustive]
#[int_enum::int_enum(u8)]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum WebhookType {
    /// Incoming Webhooks can post messages to channels with a generated token.
    Incoming = 1,
    /// Channel Follower Webhooks are internal webhooks used with Channel
    /// Following to post new messages into channels.
    ChannelFollowing = 2,
}

impl_eq_fields!(Webhook: [id, kind, guild_id, channel_id, user, name, avatar, token]);

#[cfg(test)]
mod tests {
    use serde_json::json;

    use crate::model::id::UserId;

    use super::*;

    #[test]
    fn test_deserialize_webhook() {
        let value = json!({
          "name": "test webhook",
          "type": 1,
          "channel_id": "199737254929760256",
          "token": "3d89bb7572e0fb30d8128367b3b1b44fecd1726de135cbe28a41f8b2f777c372ba2939e72279b94526ff5d1bd4358d65cf11",
          "avatar": null,
          "guild_id": "199737254929760256",
          "id": "223704706495545344",
          "user": {
            "username": "test",
            "discriminator": "7479",
            "id": "190320984123768832",
            "avatar": "b004ec1740a63ca06ae2e14c5cee11f3"
          }
        });
        let webhook = Webhook {
            id: WebhookId::from(223704706495545344),
            kind: WebhookType::Incoming,
            guild_id: Some(GuildId::from(199737254929760256)),
            channel_id: ChannelId::from(199737254929760256),
            user: Some(User {
                id: UserId::from(190320984123768832),
                name: "test".to_owned(),
                discriminator: "7479".parse().unwrap(),
                avatar: Some("b004ec1740a63ca06ae2e14c5cee11f3".to_owned()),
                bot: false,
                system: false,
            }),
            name: Some("test webhook".to_owned()),
            avatar: None,
            token: Some("3d89bb7572e0fb30d8128367b3b1b44fecd1726de135cbe28a41f8b2f777c372ba2939e72279b94526ff5d1bd4358d65cf11".to_owned()),
        };

        let deserialized = Webhook::deserialize(&value).unwrap();
        assert_eq_fields!(webhook, deserialized);
    }
}
