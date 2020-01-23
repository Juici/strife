use std::borrow::{Borrow, Cow};
use std::fmt::{self, Display};
use std::hash::{Hash, Hasher};
use std::str::FromStr;

use serde::de;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::model::id::EmojiId;

/// A custom guild emoji with partial information.
#[non_exhaustive]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CustomEmoji {
    /// The ID of the custom emoji.
    pub id: EmojiId,
    /// The name of the custom emoji.
    ///
    /// # Notes
    ///
    /// In `MESSAGE_REACTION_ADD` and `MESSAGE_REACTION_REMOVE` gateway
    /// events `name` may be `None` when custom emoji data is not
    /// available (for example, if it was deleted from the guild).
    pub name: Option<Cow<'static, str>>,
    /// Whether the custom emoji is animated.
    #[serde(default)]
    pub animated: bool,
}

impl CustomEmoji {
    /// Creates a custom guild emoji.
    pub fn new<I, S>(id: I, name: S, animated: bool) -> CustomEmoji
    where
        I: Into<EmojiId>,
        S: Into<Cow<'static, str>>,
    {
        CustomEmoji {
            id: id.into(),
            name: Some(name.into()),
            animated,
        }
    }
}

impl PartialEq for CustomEmoji {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for CustomEmoji {}

impl Hash for CustomEmoji {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl Display for CustomEmoji {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        /// Rendering a custom emoji doesn't require knowing the name, however
        /// if no name is provided the emoji will not render.
        ///
        /// Work around this issue by substituting unknown names with `_`.
        const UNKNOWN_NAME: &str = "_";

        if self.animated {
            f.write_str("<a:")?;
        } else {
            f.write_str("<:")?;
        }

        let name = self.name.as_ref().map(Cow::borrow).unwrap_or(UNKNOWN_NAME);
        write!(f, "{}:{}>", name, self.id)
    }
}

/// An emoji, with partial information.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum PartialEmoji {
    /// A standard unicode emoji.
    Standard(Cow<'static, str>),
    /// A custom guild emoji.
    Custom(CustomEmoji),
}

impl PartialEmoji {
    /// Creates a standard unicode emoji.
    pub fn standard<S>(emoji: S) -> PartialEmoji
    where
        S: Into<Cow<'static, str>>,
    {
        PartialEmoji::Standard(emoji.into())
    }

    /// Creates a custom guild emoji.
    pub fn custom<I, S>(id: I, name: S, animated: bool) -> PartialEmoji
    where
        I: Into<EmojiId>,
        S: Into<Cow<'static, str>>,
    {
        PartialEmoji::Custom(CustomEmoji::new(id, name, animated))
    }

    /// Returns the ID of the emoji.
    pub fn id(&self) -> Option<EmojiId> {
        match self {
            PartialEmoji::Standard(_) => None,
            PartialEmoji::Custom(emoji) => Some(emoji.id),
        }
    }

    /// Returns the name of the emoji.
    ///
    /// For standard unicode emojis, the name is the UTF-8 representation of the
    /// emoji.
    pub fn name(&self) -> Option<&str> {
        match self {
            PartialEmoji::Standard(name) => Some(name),
            PartialEmoji::Custom(emoji) => emoji.name.as_deref(),
        }
    }

    /// Returns whether the emoji is animated.
    pub fn animated(&self) -> bool {
        match self {
            PartialEmoji::Standard(_) => false,
            PartialEmoji::Custom(emoji) => emoji.animated,
        }
    }
}

impl Display for PartialEmoji {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PartialEmoji::Standard(name) => f.write_str(name),
            PartialEmoji::Custom(emoji) => emoji.fmt(f),
        }
    }
}

impl From<CustomEmoji> for PartialEmoji {
    fn from(emoji: CustomEmoji) -> Self {
        PartialEmoji::Custom(emoji)
    }
}

impl From<char> for PartialEmoji {
    fn from(ch: char) -> Self {
        PartialEmoji::standard(ch.to_string())
    }
}

impl From<String> for PartialEmoji {
    fn from(s: String) -> Self {
        PartialEmoji::standard(s)
    }
}

impl From<&'static str> for PartialEmoji {
    fn from(s: &'static str) -> Self {
        PartialEmoji::standard(s)
    }
}

impl FromStr for PartialEmoji {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(PartialEmoji::standard(s.to_owned()))
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

        match self {
            PartialEmoji::Standard(name) => {
                let mut map = serializer.serialize_map(Some(len))?;
                map.serialize_entry("id", &None::<EmojiId>)?;
                map.serialize_entry("name", name)?;
                map.end()
            }
            PartialEmoji::Custom(emoji) => emoji.serialize(serializer),
        }
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
                        let name = name.map(Cow::Owned);
                        let animated = animated.unwrap_or(false);
                        PartialEmoji::Custom(CustomEmoji { id, name, animated })
                    }
                    None => {
                        let name = name.ok_or_else(|| de::Error::missing_field("name"))?;
                        PartialEmoji::standard(name)
                    }
                })
            }
        }

        deserializer.deserialize_map(Visitor)
    }
}
