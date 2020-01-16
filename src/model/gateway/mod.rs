//! Models related to the gateway.

use serde::{Deserialize, Serialize};

use crate::model::id::UserId;
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

//#[derive(Clone, Debug, Deserialize, Serialize)]
// pub struct Presence {
//    pub user: PartialUser,
//}

impl_eq_fields!(PartialUser: (a, b) => {
    match (a, b) {
        (PartialUser::Full(a), PartialUser::Full(b)) => assert_eq_fields!(a, b),
        (PartialUser::Partial { id: id_a }, PartialUser::Partial { id: id_b }) => assert_eq_fields!(id_a, id_b),
        (a, b) => panic_ne_fields!(a, b),
    }
});

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
