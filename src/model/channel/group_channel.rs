use std::collections::HashMap;

use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

use crate::model::channel::utils::serde_recipients;
use crate::model::channel::ChannelType;
use crate::model::id::{ChannelId, MessageId, UserId};
use crate::model::user::User;

/// A group message channel between multiple [`User`]s.
///
/// [`User`]: ../../user/struct.User.html
#[non_exhaustive]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Group {
    /// The ID of the group channel.
    pub id: ChannelId,
    /// The type of the channel.
    ///
    /// This should always be [`ChannelType::Group`].
    ///
    /// [`ChannelType::Group`]: ../enum.ChannelType.html#variant.Group
    #[serde(rename = "type")]
    pub kind: ChannelType,
    /// The name of the group.
    pub name: Option<String>,
    /// The group icon hash.
    pub icon: Option<String>,
    /// The users in the group.
    #[serde(default, with = "serde_recipients")]
    pub recipients: HashMap<UserId, User>,
    /// The ID of the group owner.
    pub owner_id: UserId,
    /// The ID of the last message sent to the group.
    pub last_message_id: Option<MessageId>,
    /// When the last message with pinned.
    pub last_pin_timestamp: Option<DateTime<FixedOffset>>,
}

impl Group {
    /// Returns a reference to the owner of the group.
    ///
    /// # Notes
    ///
    /// If there is no user in `recipients` with the same ID as `owner_id`, the
    /// group is malformed and this function will return `None`.
    pub fn owner(&self) -> Option<&User> {
        self.recipients.get(&self.owner_id)
    }
}

impl_eq_fields!(Group: (a, b) => {
    assert_eq_fields!(a, b, [id, kind, name, icon, owner_id, last_message_id, last_pin_timestamp]);

    assert_eq!(a.recipients.len(), b.recipients.len());
    for (id, a_user) in a.recipients.iter() {
        let b_user = b.recipients.get(id).expect(&format!("missing user with id: {}", id));
        assert_eq_fields!(a_user, b_user);
    }
});

#[cfg(test)]
mod tests {
    use std::iter::FromIterator;

    use serde_json::json;

    use crate::model::user::Discriminator;

    use super::*;

    #[test]
    fn test_deserialize() {
        let value = json!({
          "name": "Some test channel",
          "icon": null,
          "recipients": [
            {
              "username": "test",
              "discriminator": "9999",
              "id": "82198898841029460",
              "avatar": "33ecab261d4681afa4d85a04691c4a01"
            },
            {
              "username": "test2",
              "discriminator": "9999",
              "id": "53908099506183680",
              "avatar": "a_bab14f271d565501444b2ca3be944b25"
            }
          ],
          "last_message_id": "3343820033257021450",
          "type": 3,
          "id": "319674150115710528",
          "owner_id": "82198810841029460"
        });
        let channel = Group {
            id: ChannelId::from(319674150115710528),
            kind: ChannelType::Group,
            name: Some("Some test channel".to_owned()),
            icon: None,
            recipients: HashMap::from_iter(vec![
                (
                    UserId::from(82198898841029460),
                    User {
                        id: UserId::from(82198898841029460),
                        name: "test".to_string(),
                        discriminator: Discriminator::new(9999).unwrap(),
                        avatar: Some("33ecab261d4681afa4d85a04691c4a01".to_owned()),
                        bot: false,
                        system: false,
                    },
                ),
                (
                    UserId::from(53908099506183680),
                    User {
                        id: UserId::from(53908099506183680),
                        name: "test2".to_string(),
                        discriminator: Discriminator::new(9999).unwrap(),
                        avatar: Some("a_bab14f271d565501444b2ca3be944b25".to_owned()),
                        bot: false,
                        system: false,
                    },
                ),
            ]),
            owner_id: UserId::from(82198810841029460),
            last_message_id: Some(MessageId::from(3343820033257021450)),
            last_pin_timestamp: None,
        };

        let deserialized: Group = serde_json::from_value(value).unwrap();

        assert_eq_fields!(channel, deserialized);
    }

    #[test]
    fn test_serialize() {
        let mut value = json!({
          "name": "Some test channel",
          "icon": null,
          "recipients": [
            {
              "username": "test",
              "discriminator": "9999",
              "id": "82198898841029460",
              "avatar": "33ecab261d4681afa4d85a04691c4a01",
              "bot": false,
              "system": false
            },
            {
              "username": "test2",
              "discriminator": "9999",
              "id": "53908099506183680",
              "avatar": "a_bab14f271d565501444b2ca3be944b25",
              "bot": false,
              "system": false
            }
          ],
          "last_message_id": "3343820033257021450",
          "last_pin_timestamp": null,
          "type": 3,
          "id": "319674150115710528",
          "owner_id": "53908099506183680"
        });
        let channel = Group {
            id: ChannelId::from(319674150115710528),
            kind: ChannelType::Group,
            name: Some("Some test channel".to_owned()),
            icon: None,
            recipients: HashMap::from_iter(vec![
                (
                    UserId::from(53908099506183680),
                    User {
                        id: UserId::from(53908099506183680),
                        name: "test2".to_string(),
                        discriminator: Discriminator::new(9999).unwrap(),
                        avatar: Some("a_bab14f271d565501444b2ca3be944b25".to_owned()),
                        bot: false,
                        system: false,
                    },
                ),
                (
                    UserId::from(82198898841029460),
                    User {
                        id: UserId::from(82198898841029460),
                        name: "test".to_string(),
                        discriminator: Discriminator::new(9999).unwrap(),
                        avatar: Some("33ecab261d4681afa4d85a04691c4a01".to_owned()),
                        bot: false,
                        system: false,
                    },
                ),
            ]),
            owner_id: UserId::from(53908099506183680),
            last_message_id: Some(MessageId::from(3343820033257021450)),
            last_pin_timestamp: None,
        };

        let mut serialized = serde_json::to_value(&channel).unwrap();

        // Stable sort of recipients for `assert_eq`.
        fn sort_recipients(vec: &mut Vec<serde_json::Value>) {
            vec.sort_by(|a, b| a["id"].as_str().unwrap().cmp(&b["id"].as_str().unwrap()))
        }
        sort_recipients(value["recipients"].as_array_mut().unwrap());
        sort_recipients(serialized["recipients"].as_array_mut().unwrap());

        assert_eq!(value, serialized);
    }
}
