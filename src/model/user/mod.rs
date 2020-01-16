//! Models related to [users].
//!
//! [users]: https://discordapp.com/developers/docs/resources/user#user-object

mod discriminator;
mod flags;

use serde::{Deserialize, Serialize};

use crate::model::id::UserId;
use crate::model::misc::Locale;
use crate::model::utils::is_false;

pub use self::discriminator::{Discriminator, DiscriminatorParseError};
pub use self::flags::UserFlags;

/// The client user.
#[non_exhaustive]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ClientUser {
    #[serde(flatten)]
    user: User,
    /// Whether the user has multi-factor authentication enabled on their
    /// account.
    #[serde(default)]
    pub mfa_enabled: bool,
    /// The chosen language of the user.
    #[serde(default)]
    pub locale: Locale,
    /// Whether the email on the user account is verified.
    #[serde(default)]
    pub verified: bool,
    /// The email of the user.
    #[serde(default)]
    pub email: Option<String>,
    /// The [flags] on the user account.
    ///
    /// [flags]: struct.UserFlags.html
    #[serde(default)]
    pub flags: UserFlags,
    /// The [type of Nitro subscription][type] on the user account.
    ///
    /// [type]: struct.PremiumType.html
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub premium_type: Option<PremiumType>,
}
wrap!(ClientUser => mut user: User);

/// A user.
#[non_exhaustive]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct User {
    /// The ID of the user.
    pub id: UserId,
    /// The username of the user, not unique.
    #[serde(rename = "username")]
    pub name: String,
    /// The 4-digit discriminator tag of the user.
    pub discriminator: Discriminator,
    /// The avatar hash of the user.
    pub avatar: Option<String>,
    /// Whether the user is a bot.
    #[serde(default, skip_serializing_if = "is_false")]
    pub bot: bool,
    /// Whether the user is an Official Discord System user (part of the urgent
    /// message system).
    #[serde(default, skip_serializing_if = "is_false")]
    pub system: bool,
}

/// The level of premium a [`User`] has.
///
/// [`User`]: struct.User.html
#[non_exhaustive]
#[int_enum::int_enum(u8)]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum PremiumType {
    /// Nitro Classic.
    NitroClassic = 1,
    /// Nitro.
    Nitro = 2,
}

impl_eq_fields!(ClientUser: [user, mfa_enabled, locale, verified, email, flags, premium_type]);
impl_eq_fields!(User: [id, name, discriminator, avatar, bot, system]);

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn test_deserialize_user() {
        let value = json!({
            "id": "80351110224678912",
            "username": "Nelly",
            "discriminator": "1337",
            "avatar": "8342729096ea3675442027381ff50dfe",
        });
        let user = User {
            id: UserId::from(80351110224678912),
            name: "Nelly".to_owned(),
            discriminator: "1337".parse().unwrap(),
            avatar: Some("8342729096ea3675442027381ff50dfe".to_owned()),
            bot: bool::default(),
            system: bool::default(),
        };

        let deserialized = User::deserialize(&value).unwrap();
        assert_eq_fields!(user, deserialized);
    }

    #[test]
    fn test_serialize_user() {
        let value = json!({
            "id": "225336713231204353",
            "username": "Juici",
            "avatar": "a_e8b3a198dab6af59aacd1072bbedb255",
            "discriminator": "0001"
        });
        let user = User {
            id: UserId::from(225336713231204353),
            name: "Juici".to_owned(),
            discriminator: "0001".parse().unwrap(),
            avatar: Some("a_e8b3a198dab6af59aacd1072bbedb255".to_owned()),
            bot: false,
            system: false,
        };

        assert_eq!(value, serde_json::to_value(&user).unwrap());
    }

    #[test]
    fn test_deserialize_client_user1() {
        let value = json!({
            "id": "82198898841029460",
            "username": "test",
            "discriminator": "9999",
            "avatar": "33ecab261d4681afa4d85a04691c4a01",
            "bot": false,
            "mfa_enabled": true,
            "locale": "en-US",
            "verified": true,
            "email": "test@example.com",
            "flags": 64,
            "premium_type": 1,
        });
        let user = ClientUser {
            user: User {
                id: UserId::from(82198898841029460),
                name: "test".to_owned(),
                discriminator: "9999".parse().unwrap(),
                avatar: Some("33ecab261d4681afa4d85a04691c4a01".to_owned()),
                bot: false,
                system: bool::default(),
            },
            mfa_enabled: true,
            locale: Locale::from_static("en-US"),
            verified: true,
            email: Some("test@example.com".to_owned()),
            flags: UserFlags::from_bits(64).unwrap(),
            premium_type: Some(PremiumType::NitroClassic),
        };

        let deserialized = ClientUser::deserialize(&value).unwrap();
        assert_eq_fields!(user, deserialized);
    }

    #[test]
    fn test_deserialize_client_user2() {
        let value = json!({
            "id": "82198898841029460",
            "username": "some bot",
            "discriminator": "0369",
            "avatar": null,
            "bot": true,
            "mfa_enabled": true,
            "locale": "en-US",
            "verified": true,
            "email": null,
            "flags": 0,
        });
        let user = ClientUser {
            user: User {
                id: UserId::from(82198898841029460),
                name: "some bot".to_owned(),
                discriminator: "0369".parse().unwrap(),
                avatar: None,
                bot: true,
                system: bool::default(),
            },
            mfa_enabled: true,
            locale: Locale::from_static("en-US"),
            verified: true,
            email: None,
            flags: UserFlags::NONE,
            premium_type: None,
        };

        let deserialized = ClientUser::deserialize(&value).unwrap();
        assert_eq_fields!(user, deserialized);
    }

    #[test]
    fn test_serialize_client_user() {
        let value = json!({
            "id": "82198898841029460",
            "username": "some bot",
            "discriminator": "0369",
            "avatar": null,
            "bot": true,
            "mfa_enabled": true,
            "locale": "en-US",
            "verified": true,
            "email": null,
            "flags": 0
        });
        let user = ClientUser {
            user: User {
                id: UserId::from(82198898841029460),
                name: "some bot".to_owned(),
                discriminator: "0369".parse().unwrap(),
                avatar: None,
                bot: true,
                system: false,
            },
            mfa_enabled: true,
            locale: Locale::from_static("en-US"),
            verified: true,
            email: None,
            flags: UserFlags::NONE,
            premium_type: None,
        };

        assert_eq!(value, serde_json::to_value(&user).unwrap());
    }
}
