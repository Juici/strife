use std::fmt::{self, Display};

use serde::{Deserialize, Serialize};

use crate::model::id::EmojiId;

/// An emoji, either a standard emoji or a custom guild emoji.
#[non_exhaustive]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Emoji {
    /// The ID of the emoji, standard emojis have no ID.
    pub id: Option<EmojiId>,
    /// The name of the emoji.
    pub name: String,
    // TODO: roles
    // TODO: user
    /// Whether the name requires colons to be used by a client.
    #[serde(default)]
    pub require_colons: bool,
    /// Whether the emoji was created by an integration service.
    #[serde(default)]
    pub managed: bool,
    /// Whether the emoji is animated.
    #[serde(default)]
    pub animated: bool,
}

impl Emoji {
    /// Returns `true` if the emoji is a custom guild emoji.
    pub fn is_custom(&self) -> bool {
        self.id.is_some()
    }
}

impl PartialEq for Emoji {
    fn eq(&self, other: &Self) -> bool {
        match &self.id {
            Some(_) => self.id == other.id,
            None => self.name == other.name,
        }
    }
}

impl Eq for Emoji {}

impl Display for Emoji {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.require_colons {
            match self.id {
                Some(id) if self.animated => write!(f, "<:a:{}:{}>", &self.name, id),
                Some(id) => write!(f, "<:{}:{}>", &self.name, id),
                None => write!(f, ":{}:", &self.name),
            }
        } else {
            f.write_str(&self.name)
        }
    }
}

#[cfg(test)]
mod tests {
    use serde_json::{json, Value};

    use super::*;

    fn deserialize_emoji(value: &Value) -> Emoji {
        Emoji::deserialize(value).expect("deserialized emoji")
    }

    #[test]
    fn test_serialize_deserialize() {
        let val = json!({
          "id": "41771983429993937",
          "name": "LUL",
          "roles": ["41771983429993000", "41771983429993111"],
          "user": {
            "username": "Luigi",
            "discriminator": "0002",
            "id": "96008815106887111",
            "avatar": "5500909a3274e1812beb4e8de6631111"
          },
          "require_colons": true,
          "managed": false,
          "animated": false
        });

        let deserialized = deserialize_emoji(&val);
        let _serialized = serde_json::to_string(&deserialized).expect("serialized emoji");
    }

    #[test]
    fn test_standard() {
        let val = json!({
          "id": null,
          "name": "🔥"
        });

        let deserialized = deserialize_emoji(&val);
        assert!(!deserialized.is_custom())
    }

    #[test]
    fn test_custom() {
        let val = json!({
          "id": "41771983429993937",
          "name": "LUL"
        });

        let deserialized = deserialize_emoji(&val);
        assert!(deserialized.is_custom())
    }
}
