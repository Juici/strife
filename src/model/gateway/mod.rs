//! Models related to the gateway.

use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

use crate::model::gateway::activity::Activity;
use crate::model::id::{GuildId, RoleId, UserId};
use crate::model::user::User;

pub mod activity;

/// A user with possibly only partial information.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum PartialUser {
    /// A full user object.
    Full(User),
    /// A partial user object with only the `id` field.
    #[non_exhaustive]
    Partial {
        /// The ID of the user.
        id: UserId,
    },
}

/// The online status of a [`User`] in a [`Presence`].
///
/// [`User`]: ../user/struct.User.html
/// [`Presence`]: struct.Presence.html
#[derive(Clone, Copy, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum OnlineStatus {
    /// Idle.
    #[serde(rename = "idle")]
    Idle,
    /// Do not disturb.
    #[serde(rename = "dnd")]
    DoNotDisturb,
    /// Online.
    #[serde(rename = "online")]
    Online,
    /// Offline or invisible.
    #[serde(rename = "offline")]
    Offline,
}

impl OnlineStatus {
    fn is_offline(&self) -> bool {
        *self == OnlineStatus::Offline
    }
}

impl Default for OnlineStatus {
    fn default() -> Self {
        OnlineStatus::Offline
    }
}

/// Statuses of the active sessions for each platform for a [`User`].
///
/// [`User`]: ../user/struct.User.html
#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct ClientStatus {
    /// The status set for an active desktop (Windows, Linux, Mac) application
    /// session.
    #[serde(default, skip_serializing_if = "OnlineStatus::is_offline")]
    pub desktop: OnlineStatus,
    /// The status set for an active mobile (iOS, Android) application session.
    #[serde(default, skip_serializing_if = "OnlineStatus::is_offline")]
    pub mobile: OnlineStatus,
    /// The status set for an active web (browser, bot account) application
    /// session.
    #[serde(default, skip_serializing_if = "OnlineStatus::is_offline")]
    pub web: OnlineStatus,
}

/// The presence state of a [`User`] in a [`Guild`].
///
/// [`User`]: ../user/struct.User.html
/// [`Guild`]: ../guild/struct.Guild.html
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Presence {
    /// The user the presence relates to.
    pub user: PartialUser,
    /// The roles of the user.
    pub roles: Vec<RoleId>,
    /// The current activity of the user.
    pub game: Option<Activity>,
    /// The ID of the guild.
    pub guild_id: GuildId,
    /// The online status of the user.
    pub status: OnlineStatus,
    /// The current activities of the user.
    pub activities: Vec<Activity>,
    /// The platform dependent status of the user.
    pub client_status: ClientStatus,
    /// When the user Nitro boosted the guild.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub premium_since: Option<DateTime<FixedOffset>>,
    /// The nickname of the user in the guild, if set.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nick: Option<String>,
}

impl_eq_fields!(PartialUser: (a, b) => {
    match (a, b) {
        (PartialUser::Full(a), PartialUser::Full(b)) => assert_eq_fields!(a, b),
        (PartialUser::Partial { id: id_a }, PartialUser::Partial { id: id_b }) => assert_eq_fields!(id_a, id_b),
        (a, b) => panic_ne_fields!(a, b),
    }
});
impl_eq_fields!(Presence: [
    user,
    roles,
    game,
    guild_id,
    status,
    activities,
    client_status,
    premium_since,
    nick
]);

#[cfg(test)]
mod tests {
    use serde_json::json;

    use crate::model::user::Discriminator;

    use super::*;

    #[test]
    fn test_deserialize_partial_user() {
        let value = json!({
            "id": "80351110224678912"
        });
        let user = PartialUser::Partial {
            id: UserId::from(80351110224678912),
        };

        let deserialized = PartialUser::deserialize(&value).unwrap();
        assert_eq_fields!(user, deserialized);
    }

    #[test]
    fn test_serialize_partial_user() {
        let value = json!({
            "id": "80351110224678912"
        });
        let user = PartialUser::Partial {
            id: UserId::from(80351110224678912),
        };

        assert_eq!(value, serde_json::to_value(&user).unwrap());
    }

    #[test]
    fn test_deserialize_full_user() {
        let value = json!({
            "id": "80351110224678912",
            "username": "Nelly",
            "discriminator": "1337",
            "avatar": "8342729096ea3675442027381ff50dfe",
        });
        let user = PartialUser::Full(User {
            id: UserId::from(80351110224678912),
            name: "Nelly".to_owned(),
            discriminator: Discriminator::new(1337).unwrap(),
            avatar: Some("8342729096ea3675442027381ff50dfe".to_owned()),
            bot: false,
            system: false,
        });

        let deserialized = PartialUser::deserialize(&value).unwrap();
        assert_eq_fields!(user, deserialized);
    }

    #[test]
    fn test_serialize_full_user() {
        let value = json!({
            "id": "80351110224678912",
            "username": "Nelly",
            "discriminator": "1337",
            "avatar": "8342729096ea3675442027381ff50dfe",
            "bot": false,
            "system": false
        });
        let user = PartialUser::Full(User {
            id: UserId::from(80351110224678912),
            name: "Nelly".to_owned(),
            discriminator: Discriminator::new(1337).unwrap(),
            avatar: Some("8342729096ea3675442027381ff50dfe".to_owned()),
            bot: false,
            system: false,
        });

        assert_eq!(value, serde_json::to_value(&user).unwrap());
    }
}
