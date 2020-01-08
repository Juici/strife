/// Serde mapping functions between `User` sequence (with a known single entry)
/// and `User`.
pub mod serde_recipient {
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

/// Serde mapping functions between `User` sequence and `HashMap<UserId, User>`.
pub mod serde_recipients {
    use std::collections::HashMap;
    use std::fmt;

    use serde::de;
    use serde::{Deserializer, Serializer};

    use crate::model::id::UserId;
    use crate::model::user::User;

    pub fn deserialize<'de, D>(deserializer: D) -> Result<HashMap<UserId, User>, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct Visitor;

        impl<'de> de::Visitor<'de> for Visitor {
            type Value = HashMap<UserId, User>;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.write_str("a sequence of users")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: de::SeqAccess<'de>,
            {
                let mut map = HashMap::with_capacity(seq.size_hint().unwrap_or_default());

                while let Some(user) = seq.next_element::<User>()? {
                    if let Some(existing) = map.insert(user.id, user) {
                        return Err(de::Error::custom(format_args!(
                            "duplicate recipient user: {}",
                            existing.id
                        )));
                    }
                }

                Ok(map)
            }
        }

        deserializer.deserialize_seq(Visitor)
    }

    pub fn serialize<S>(
        recipients: &HashMap<UserId, User>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.collect_seq(recipients.values())
    }
}
