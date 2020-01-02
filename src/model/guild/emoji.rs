use std::fmt::{self, Display};
use std::hash::{Hash, Hasher};

use serde::de;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::model::id::EmojiId;
use crate::model::user::User;

/// An emoji, with partial information.
#[derive(Clone, Debug)]
pub enum PartialEmoji {
    /// A standard unicode emoji.
    Standard(String),
    /// A custom guild emoji.
    #[non_exhaustive]
    Custom {
        /// The ID of the custom emoji.
        id: EmojiId,
        /// The name of the custom emoji.
        ///
        /// # Notes
        ///
        /// In `MESSAGE_REACTION_ADD` and `MESSAGE_REACTION_REMOVE` gateway
        /// events `name` may be `None` when custom emoji data is not
        /// available (for example, if it was deleted from the guild).
        name: Option<String>,
        /// Whether the custom emoji is animated.
        animated: bool,
    },
}

impl PartialEmoji {
    /// Creates a standard unicode emoji.
    pub fn standard<S: Into<String>>(emoji: S) -> PartialEmoji {
        PartialEmoji::Standard(emoji.into())
    }

    /// Creates a custom guild emoji.
    pub fn custom<I, S>(id: I, name: S, animated: bool) -> PartialEmoji
    where
        I: Into<EmojiId>,
        S: Into<String>,
    {
        PartialEmoji::Custom {
            id: id.into(),
            name: Some(name.into()),
            animated,
        }
    }

    /// Returns the ID of the emoji.
    pub fn id(&self) -> Option<EmojiId> {
        match *self {
            PartialEmoji::Standard(_) => None,
            PartialEmoji::Custom { id, .. } => Some(id),
        }
    }

    /// Returns the name of the emoji.
    ///
    /// For standard unicode emojis, the name is the UTF-8 representation of the
    /// emoji.
    pub fn name(&self) -> Option<&str> {
        match self {
            PartialEmoji::Standard(name) => Some(name),
            PartialEmoji::Custom { name, .. } => name.as_deref(),
        }
    }

    /// Returns whether the emoji is animated.
    pub fn animated(&self) -> bool {
        match *self {
            PartialEmoji::Standard(_) => false,
            PartialEmoji::Custom { animated, .. } => animated,
        }
    }
}

impl Display for PartialEmoji {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        /// Rendering a custom emoji doesn't require knowing the name, however
        /// if no name is provided the emoji will not render.
        ///
        /// Work around this issue by substituting unknown names with `_`.
        const UNKNOWN_NAME: &str = "_";

        match *self {
            PartialEmoji::Standard(ref name) => f.write_str(name),
            PartialEmoji::Custom {
                id,
                ref name,
                animated,
            } => {
                if animated {
                    f.write_str("<a:")?;
                } else {
                    f.write_str("<:")?;
                }
                write!(f, "{}:{}>", name.as_deref().unwrap_or(UNKNOWN_NAME), id)
            }
        }
    }
}

impl PartialEq for PartialEmoji {
    fn eq(&self, other: &Self) -> bool {
        match self {
            PartialEmoji::Standard(name) => match other {
                PartialEmoji::Standard(other_name) => name == other_name,
                PartialEmoji::Custom { .. } => false,
            },
            PartialEmoji::Custom { id, .. } => match other {
                PartialEmoji::Standard(_) => false,
                PartialEmoji::Custom { id: other_id, .. } => id == other_id,
            },
        }
    }
}

impl Eq for PartialEmoji {}

impl Hash for PartialEmoji {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            PartialEmoji::Standard(name) => name.hash(state),
            PartialEmoji::Custom { id, .. } => id.hash(state),
        }
    }
}

impl Serialize for PartialEmoji {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use serde::ser::SerializeMap;

        let len = match self {
            PartialEmoji::Standard(_) => 2,
            PartialEmoji::Custom { .. } => 3,
        };

        let mut map = serializer.serialize_map(Some(len))?;
        match self {
            PartialEmoji::Standard(name) => {
                map.serialize_entry("id", &None::<EmojiId>)?;
                map.serialize_entry("name", name)?;
            }
            PartialEmoji::Custom { id, name, animated } => {
                map.serialize_entry("id", id)?;
                map.serialize_entry("name", name)?;
                map.serialize_entry("animated", animated)?;
            }
        };
        map.end()
    }
}

impl<'de> Deserialize<'de> for PartialEmoji {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(field_identifier, rename_all = "snake_case")]
        enum Field {
            Id,
            Name,
            Animated,
        }

        struct Visitor;

        impl<'de> de::Visitor<'de> for Visitor {
            type Value = PartialEmoji;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.write_str("PartialEmoji")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: de::MapAccess<'de>,
            {
                let mut id = None;
                let mut name = None;
                let mut animated = None;

                while let Some(key) = map.next_key().ok().flatten() {
                    match key {
                        Field::Id => {
                            if id.is_some() {
                                return Err(de::Error::duplicate_field("id"));
                            }
                            id = map.next_value::<Option<EmojiId>>()?;
                        }
                        Field::Name => {
                            if name.is_some() {
                                return Err(de::Error::duplicate_field("name"));
                            }
                            name = map.next_value::<Option<String>>()?;
                        }
                        Field::Animated => {
                            if animated.is_some() {
                                return Err(de::Error::duplicate_field("animated"));
                            }
                            animated = Some(map.next_value::<bool>()?);
                        }
                    }
                }

                Ok(match id {
                    Some(id) => {
                        let animated = animated.unwrap_or_default();
                        PartialEmoji::Custom { id, name, animated }
                    }
                    None => {
                        let name = name.ok_or(de::Error::missing_field("name"))?;
                        PartialEmoji::Standard(name)
                    }
                })
            }
        }

        deserializer.deserialize_map(Visitor)
    }
}

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

    macro_rules! assert_eq_fields {
        ($left:expr, $right:expr, [$($field:ident),* $(,)*]) => {$(
            assert_eq!($left.$field, $right.$field);
        )*};
    }

    #[test]
    fn test_deserialize_standard() {
        let value = json!({
            "id": null,
            "name": "🔥",
        });
        let emoji = PartialEmoji::standard("🔥");

        assert_eq!(emoji, serde_json::from_value(value).unwrap());
    }

    #[test]
    fn test_serialize_standard() {
        let value = json!({
            "id": null,
            "name": "🔥",
        });
        let emoji = PartialEmoji::standard("🔥");

        assert_eq!(value, serde_json::to_value(emoji).unwrap());
    }

    #[test]
    fn test_deserialize_custom() {
        let value = json!({
            "id": "41771983429993937",
            "name": "LUL",
        });
        let emoji = PartialEmoji::custom(41771983429993937, "LUL", false);

        assert_eq!(emoji, serde_json::from_value(value).unwrap());
    }

    #[test]
    fn test_serialize_custom() {
        let value = json!({
            "id": "41771983429993937",
            "name": "LUL",
            "animated": true,
        });
        let emoji = PartialEmoji::custom(41771983429993937, "LUL", true);

        assert_eq!(value, serde_json::to_value(emoji).unwrap());
    }

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
