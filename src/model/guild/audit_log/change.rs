use std::collections::HashMap;
use std::fmt;

use serde::de;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::model::guild::settings::{
    ExplicitContentFilterLevel, MessageNotificationLevel, MfaLevel,
};
use crate::model::id::{ChannelId, UserId};
use crate::model::voice::VoiceRegionId;

macro_rules! changes {
    (
        $(#[$enum_attr:meta])*
        $vis:vis enum $enum_name:ident {
            $(
                $(#[doc = $variant_doc:expr])*
                #[key = $variant_key:literal]
                $variant_name:ident : $variant_type:ty
            ),+ $(,)?
        }
    ) => {
        $(#[$enum_attr])*
        #[derive(Clone, Debug, Deserialize, Serialize)]
        #[serde(tag = "key")]
        $vis enum $enum_name {
            $(
                $(#[doc = $variant_doc])*
                #[serde(rename = $variant_key)]
                $variant_name {
                    /// The old value.
                    #[serde(default, skip_serializing_if = "Option::is_none")]
                    old_value: Option<$variant_type>,
                    /// The new value.
                    #[serde(default, skip_serializing_if = "Option::is_none")]
                    new_value: Option<$variant_type>,
                }
            ),+
        }

        impl $enum_name {
            // TODO: Make const when `const_if_match` is stabilised.
            fn key(&self) -> &'static str {
                // Hack to error on duplicate keys.
                const __ASSERT_NO_DUPE_KEY: fn() = || {
                    #[forbid(unreachable_patterns)]
                    match "" {
                        $(
                            $variant_key => {}
                        )+
                        _ => {}
                    }
                };

                match self {
                    $(
                        $enum_name::$variant_name { .. } => { $variant_key }
                    )+
                }
            }
        }

        impl_eq_fields!($enum_name: (a, b) => {
            match (a, b) {
                $(
                    (
                        $enum_name::$variant_name { old_value: a_old, new_value: a_new },
                        $enum_name::$variant_name { old_value: b_old, new_value: b_new },
                    ) => {
                        assert_eq_fields!(a_old, b_old);
                        assert_eq_fields!(a_new, b_new);
                    }
                )+
                // Not same variant here, just assert keys are equal (which they aren't).
                (a, b) => {
                    assert_eq!(a.key(), b.key());
                }
            }
        });
    };
}

changes! {
    /// A change in an [`AuditLogEntry`].
    ///
    /// [`AuditLogEntry`]: ../struct.AuditLogEntry.html
    pub enum AuditLogChange {
        // Guild

        /// Name changed.
        #[key = "name"]
        Name: String,

        /// Icon changed.
        #[key = "icon_hash"]
        IconHash: String,

        /// Invite splash page artwork changed.
        #[key = "splash_hash"]
        SplashHash: String,

        /// Owner changed.
        #[key = "owner_id"]
        Owner: UserId,

        /// Voice region changed.
        #[key = "region"]
        Region: VoiceRegionId,

        /// AFK channel changed.
        #[key = "afk_channel_id"]
        AfkChannel: ChannelId,

        /// AFK timeout duration changed.
        #[key = "afk_timeout"]
        AfkTimeout: u64,

        /// Multi-factor authentication level changed.
        #[key = "mfa_level"]
        MfaLevel: MfaLevel,

        /// Filter level for blocking explicit content was changed.
        #[key = "explicit_content_filter"]
        ExplicitContentFilter: ExplicitContentFilterLevel,

        /// Default message notification level changed.
        #[key = "default_message_notifications"]
        NotificationLevel: MessageNotificationLevel,

        // Channel
    }
}

#[non_exhaustive]
#[derive(Clone, Debug)]
pub struct AuditLogChanges {
    changes: HashMap<&'static str, AuditLogChange>,
}

impl Serialize for AuditLogChanges {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use serde::ser::SerializeTuple;

        let mut changes = serializer.serialize_tuple(self.changes.len())?;

        for (_map_key, change) in &self.changes {
            changes.serialize_element(change)?;
        }

        changes.end()
    }
}

impl<'de> Deserialize<'de> for AuditLogChanges {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct Visitor;

        impl<'de> de::Visitor<'de> for Visitor {
            type Value = AuditLogChanges;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.write_str("a sequence of audit log changes")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: de::SeqAccess<'de>,
            {
                const CAUTIOUS_SIZE_HINT: usize = 128;

                let mut changes =
                    HashMap::with_capacity(seq.size_hint().unwrap_or(CAUTIOUS_SIZE_HINT));

                while let Some(change) = seq.next_element::<AuditLogChange>()? {
                    changes.insert(change.key(), change);
                }

                Ok(AuditLogChanges { changes })
            }
        }

        deserializer.deserialize_seq(Visitor)
    }
}

impl_eq_fields!(AuditLogChanges: (a, b) => {
    assert_eq_fields!(map => a.changes, b.changes);
});

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn test_deserialize_change() {
        let value = json!({
          "key": "name",
          "new_value": "example-channel"
        });
        let change = AuditLogChange::Name {
            old_value: None,
            new_value: Some("example-channel".to_owned()),
        };

        let deserialized = AuditLogChange::deserialize(&value).unwrap();
        assert_eq_fields!(change, deserialized);
    }

    #[test]
    fn test_deserialize_changes() {
        let value = json!([
          {
            "key": "name",
            "new_value": "example-channel"
          },
          {
            "key": "type",
            "new_value": 0
          },
          {
            "key": "topic",
            "new_value": "example topic"
          },
          {
            "key": "permission_overwrites",
            "new_value": []
          },
          {
            "key": "nsfw",
            "new_value": false
          },
          {
            "key": "rate_limit_per_user",
            "new_value": 0
          }
        ]);
        let changes = AuditLogChanges {
            changes: {
                let changes = vec![AuditLogChange::Name {
                    old_value: None,
                    new_value: Some("example-channel".to_owned()),
                }];

                let mut map = HashMap::with_capacity(changes.len());
                for change in changes {
                    map.insert(change.key(), change);
                }

                map
            },
        };

        let deserialized = AuditLogChanges::deserialize(&value).unwrap();
        assert_eq_fields!(changes, deserialized);
    }
}
