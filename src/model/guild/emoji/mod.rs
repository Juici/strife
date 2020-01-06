mod partial;

use std::fmt::{self, Display};

use serde::{Deserialize, Serialize};

use crate::model::user::User;

pub use self::partial::PartialEmoji;

/// An emoji.
#[non_exhaustive]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Emoji {
    /// The ID of the emoji.
    #[serde(flatten)]
    pub emoji: PartialEmoji,
    // TODO: roles
    /// The user that created the emoji.
    pub user: Option<User>,
    /// Whether the name requires colons to be used by a client.
    #[serde(default)]
    pub require_colons: bool,
    /// Whether the emoji was created by an integration service.
    #[serde(default)]
    pub managed: bool,
}

impl Display for Emoji {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.emoji.fmt(f)
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use crate::model::id::UserId;
    use crate::model::user::Discriminator;

    use super::*;

    #[test]
    fn test_deserialize_emoji() {
        let value = json!({
          "id": "41771983429993937",
          "name": "LUL",
          "roles": ["41771983429993000", "41771983429993111"],
          "user": {
            "username": "Luigi",
            "discriminator": "0002",
            "id": "96008815106887111",
            "avatar": "5500909a3274e1812beb4e8de6631111",
          },
          "require_colons": true,
          "managed": false,
          "animated": false,
        });
        let emoji = Emoji {
            emoji: PartialEmoji::custom(41771983429993937, "LUL", false),
            user: Some(User {
                id: UserId::from(96008815106887111),
                name: "Luigi".to_owned(),
                discriminator: Discriminator::new(2).unwrap(),
                avatar: Some("5500909a3274e1812beb4e8de6631111".to_owned()),
                bot: false,
                system: false,
            }),
            require_colons: true,
            managed: false,
        };

        let deserialized: Emoji = serde_json::from_value(value).unwrap();

        assert_eq_fields!(emoji, deserialized, [emoji, require_colons, managed]);
        assert_eq_fields!(
            emoji.user.as_ref().unwrap(),
            deserialized.user.as_ref().unwrap(),
            [id, name, discriminator, avatar, bot, system]
        );
    }

    // TODO: Enable test when `roles` is added to `Emoji`.
    #[test]
    #[ignore]
    fn test_serialize_emoji() {
        let value = json!({
          "id": "41771983429993937",
          "name": "LUL",
          "roles": ["41771983429993000", "41771983429993111"],
          "user": {
            "username": "Luigi",
            "discriminator": "0002",
            "id": "96008815106887111",
            "avatar": null,
            "bot": false,
            "system": false,
          },
          "require_colons": true,
          "managed": false,
          "animated": true,
        });
        let emoji = Emoji {
            emoji: PartialEmoji::custom(41771983429993937, "LUL", true),
            user: Some(User {
                id: UserId::from(96008815106887111),
                name: "Luigi".to_owned(),
                discriminator: Discriminator::new(2).unwrap(),
                avatar: None,
                bot: false,
                system: false,
            }),
            require_colons: true,
            managed: false,
        };

        assert_eq!(value, serde_json::to_value(emoji).unwrap());
    }
}
