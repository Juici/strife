use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

use crate::model::channel::ChannelType;
use crate::model::id::{ChannelId, MessageId};
use crate::model::user::User;

/// A direct message channel between the [`ClientUser`] and another [`User`].
///
/// [`ClientUser`]: ../../user/struct.ClientUser.html
/// [`User`]: ../../user/struct.User.html
#[non_exhaustive]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DMChannel {
    /// The ID of the channel.
    pub id: ChannelId,
    /// The type of the channel.
    ///
    /// This should always be [`ChannelType::Private`].
    ///
    /// [`ChannelType::Private`]: ../enum.ChannelType.html#variant.Private
    #[serde(rename = "type")]
    pub(crate) kind: ChannelType,
    /// The recipient to the direct message channel.
    #[serde(rename = "recipients", with = "serde_recipient")]
    pub recipient: User,
    /// The ID of the last message sent to the channel.
    #[serde(default)]
    pub last_message_id: Option<MessageId>,
    /// When the last message was pinned.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_pin_timestamp: Option<DateTime<FixedOffset>>,
}

/// Serde mapping functions between `User` sequence (with a known single entry)
/// and `User`.
mod serde_recipient {
    use serde::ser::SerializeTuple;
    use serde::{Deserialize, Deserializer, Serializer};

    use crate::model::user::User;

    pub fn deserialize<'de, D>(deserializer: D) -> Result<User, D::Error>
    where
        D: Deserializer<'de>,
    {
        let [recipient] = <[User; 1]>::deserialize(deserializer)?;
        Ok(recipient)
    }

    pub fn serialize<S>(recipient: &User, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut tuple = serializer.serialize_tuple(1)?;
        tuple.serialize_element(recipient)?;
        tuple.end()
    }
}

impl_eq_fields!(DMChannel: [id, kind, recipient, last_message_id, last_pin_timestamp]);

#[cfg(test)]
mod tests {
    use chrono::TimeZone;
    use serde_json::json;

    use crate::model::channel::Channel;
    use crate::model::id::UserId;
    use crate::model::user::Discriminator;

    use super::*;

    #[test]
    fn test_deserialize() {
        let value = json!({
          "last_message_id": "3343820033257021450",
          "type": 1,
          "id": "319674150115610528",
          "recipients": [
            {
              "username": "test",
              "discriminator": "9999",
              "id": "82198898841029460",
              "avatar": "33ecab261d4681afa4d85a04691c4a01"
            }
          ]
        });
        let channel = DMChannel {
            id: ChannelId::from(319674150115610528),
            kind: ChannelType::Private,
            recipient: User {
                id: UserId::from(82198898841029460),
                name: "test".to_owned(),
                discriminator: Discriminator::new(9999).unwrap(),
                avatar: Some("33ecab261d4681afa4d85a04691c4a01".to_owned()),
                bot: false,
                system: false,
            },
            last_message_id: Some(MessageId::from(3343820033257021450)),
            last_pin_timestamp: None,
        };

        let deserialized = DMChannel::deserialize(&value).unwrap();
        assert_eq_fields!(channel, deserialized);

        let channel = Channel::DM(channel);
        let deserialized = Channel::deserialize(&value).unwrap();
        assert_eq_fields!(channel, deserialized);
    }

    #[test]
    fn test_serialize() {
        let value = json!({
          "id": "550383196278358041",
          "last_message_id": "663411909835620352",
          "last_pin_timestamp": "2020-01-05T16:02:04.179+00:00",
          "type": 1,
          "recipients": [
            {
              "id": "225336713231204353",
              "username": "Juici",
              "avatar": "a_e8b3a198dab6af59aacd1072bbedb255",
              "discriminator": "0001"
            }
          ]
        });
        let channel = DMChannel {
            id: ChannelId::from(550383196278358041),
            kind: ChannelType::Private,
            recipient: User {
                id: UserId::from(225336713231204353),
                name: "Juici".to_owned(),
                discriminator: Discriminator::new(1).unwrap(),
                avatar: Some("a_e8b3a198dab6af59aacd1072bbedb255".to_owned()),
                bot: false,
                system: false,
            },
            last_message_id: Some(MessageId::from(663411909835620352)),
            last_pin_timestamp: Some(
                FixedOffset::east(0)
                    .ymd(2020, 1, 5)
                    .and_hms_milli(16, 2, 4, 179),
            ),
        };

        assert_eq!(value, serde_json::to_value(&channel).unwrap());
    }
}
