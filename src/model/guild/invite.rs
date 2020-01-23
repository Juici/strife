//! Models related to invites.

use serde::{Deserialize, Serialize};

use crate::model::channel::guild::PartialGuildChannel;
use crate::model::guild::PartialGuild;
use crate::model::user::User;

#[allow(missing_docs)]
#[non_exhaustive]
#[int_enum::int_enum(u8)]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum TargetUserType {
    Stream = 1,
}

/// A code that when used, adds a [`User`] to a [`Guild`].
///
/// [`User`]: ../../user/struct.User.html
/// [`Guild`]: ../struct.Guild.html
#[non_exhaustive]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Invite {
    /// The invite code (unique ID).
    pub code: String,
    /// The guild the invite is for.
    pub guild: PartialGuild,
    /// The channel the invite is for.
    pub channel: PartialGuildChannel,
    /// The user the invite is targeted at.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_user: Option<User>,
    /// The type of user the invite is targeted at.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_user_type: Option<TargetUserType>,
    /// An approximate count of online members.
    #[serde(rename = "approximate_presence_count")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub presence_count: Option<u64>,
    /// An approximate count of total members.
    #[serde(rename = "approximate_member_count")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub member_count: Option<u64>,
}

impl_eq_fields!(Invite: [
    code,
    guild,
    channel,
    target_user,
    target_user_type,
    presence_count,
    member_count
]);

#[cfg(test)]
mod tests {
    use serde_json::json;

    use crate::model::channel::ChannelType;
    use crate::model::id::{ChannelId, GuildId, UserId};

    use super::*;

    #[test]
    fn test_deserialize_invite() {
        let value = json!({
          "code": "0vCdhLbwjZZTWZLD",
          "guild": {
            "id": "165176875973476352",
            "name": "CS:GO Fraggers Only",
            "splash": null,
            "icon": null
          },
          "channel": {
            "id": "165176875973476352",
            "name": "illuminati",
            "type": 0
          },
          "target_user": {
            "id": "165176875973476352",
            "username": "bob",
            "avatar": "deadbeef",
            "discriminator": "1234"
          },
          "target_user_type": 1
        });
        let invite = Invite {
            code: "0vCdhLbwjZZTWZLD".to_owned(),
            guild: PartialGuild {
                id: GuildId::from(165176875973476352),
                name: "CS:GO Fraggers Only".to_string(),
                icon: None,
                splash: None,
                owner: false,
                permissions: None,
            },
            channel: PartialGuildChannel {
                id: ChannelId::from(165176875973476352),
                kind: ChannelType::Text,
                name: "illuminati".to_owned(),
            },
            target_user: Some(User {
                id: UserId::from(165176875973476352),
                name: "bob".to_owned(),
                discriminator: "1234".parse().unwrap(),
                avatar: Some("deadbeef".to_owned()),
                bot: false,
                system: false,
            }),
            target_user_type: Some(TargetUserType::Stream),
            presence_count: None,
            member_count: None,
        };

        let deserialized = Invite::deserialize(&value).unwrap();
        assert_eq_fields!(invite, deserialized);
    }
}
