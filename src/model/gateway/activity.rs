//! Models related to activities in Rich Presence.

use bitflags::bitflags;
use chrono::{DateTime, Utc};
use serde::de;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::model::id::{ApplicationId, EmojiId};
use crate::model::utils::{is_false, serde_option_timestamp, U8Visitor};

/// Timestamps for start and end of an [`Activity`].
///
/// [`Activity`]: struct.Activity.html
#[non_exhaustive]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ActivityTimestamps {
    /// When the activity started.
    #[serde(with = "serde_option_timestamp")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start: Option<DateTime<Utc>>,
    /// When the activity ends.
    #[serde(with = "serde_option_timestamp")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end: Option<DateTime<Utc>>,
}

/// The type of an [`Activity`].
///
/// [`Activity`]: struct.Activity.html
#[non_exhaustive]
#[int_enum::int_enum(u8)]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ActivityType {
    /// Format: `Playing {`[`name`]`}`.
    ///
    /// [`name`]: struct.Activity.html#structfield.name
    Game = 0,
    /// Format: `Streaming {`[`details`]`}`.
    ///
    /// [`details`]: struct.Activity.html#structfield.details
    Streaming = 1,
    /// Format: `Listening to {`[`name`]`}`.
    ///
    /// [`name`]: struct.Activity.html#structfield.name
    Listening = 2,
    /// Format: `{`[`emoji`]`} {`[`name`]`}`.
    ///
    /// [`emoji`]: struct.Activity.html#structfield.emoji
    /// [`name`]: struct.Activity.html#structfield.name
    Custom = 3,
}

/// An emoji in a [`Custom`] status for an [`Activity`]
///
/// [`Custom`]: enum.ActivityType.html#variant.Custom
/// [`Activity`]: struct.Activity.html
#[non_exhaustive]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ActivityEmoji {
    /// The name of the emoji.
    pub name: String,
    /// The ID of the emoji.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<EmojiId>,
    /// Whether the emoji is animated.
    #[serde(default, skip_serializing_if = "is_false")]
    pub animated: bool,
}

/// An in-game party in an [`Activity`].
///
/// [`Activity`]: struct.Activity.html
#[non_exhaustive]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ActivityParty {
    /// The ID of the party.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The current size and max size of the party.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<[u64; 2]>,
}

impl ActivityParty {
    /// The current size of the party.
    pub fn current_size(&self) -> Option<u64> {
        self.size.as_ref().map(|size| size[0])
    }

    /// The maximum size of the party.
    pub fn max_size(&self) -> Option<u64> {
        self.size.as_ref().map(|size| size[1])
    }
}

/// Images and accompanying hover texts for an [`Activity`].
///
/// [`Activity`]: struct.Activity.html
#[non_exhaustive]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ActivityAssets {
    /// The ID for a large asset of the activity, usually a snowflake.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub large_image: Option<String>,
    /// The text displayed when hovering over the [large image] of the activity.
    ///
    /// [large image]: #structfield.large_image
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub large_text: Option<String>,
    /// The ID for a small asset of the activity, usually a snowflake.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub small_image: Option<String>,
    /// The text displayed when hovering over the [small image] of the activity.
    ///
    /// [small image]: #structfield.small_image
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub small_text: Option<String>,
}

/// Secrets for joining or spectating an [`Activity`].
///
/// [`Activity`]: struct.Activity.html
#[non_exhaustive]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ActivitySecrets {
    /// The secret for joining a party.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub join: Option<String>,
    /// The secret for spectating a game.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spectate: Option<String>,
    /// The secret for a specific instanced match.
    #[serde(rename = "match")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub game: Option<String>,
}

bitflags! {
    /// A set of flags describing an activity.
    #[derive(Default)]
    pub struct ActivityFlags: u8 {
        /// The activity is an instance activity.
        const INSTANCE = 1 << 0;
        /// The activity is joinable.
        const JOIN = 1 << 1;
        /// The activity can be spectated.
        const SPECTATE = 1 << 2;
        /// A request can be sent to join the activity party.
        const JOIN_REQUEST = 1 << 3;
        /// The activity can be synced.
        const SYNC = 1 << 4;
        /// The activity can be played.
        const PLAY = 1 << 5;
    }
}

impl Serialize for ActivityFlags {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u8(self.bits())
    }
}

impl<'de> Deserialize<'de> for ActivityFlags {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let bits = deserializer.deserialize_any(U8Visitor)?;
        match ActivityFlags::from_bits(bits) {
            Some(perms) => Ok(perms),
            None => {
                let unknown: u8 = bits & !ActivityFlags::all().bits();
                Err(de::Error::custom(format!(
                    "unknown permissions bits {:b} in {:b}",
                    unknown, bits
                )))
            }
        }
    }
}

/// An activity.
#[non_exhaustive]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Activity {
    /// The name of the activity.
    pub name: String,
    /// The type of activity.
    #[serde(rename = "type")]
    pub kind: ActivityType,
    /// The URL of the stream.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    // TODO: Look into `created_at` field.
    /// Timestamps for when the game started or ends.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timestamps: Option<ActivityTimestamps>,
    /// The application ID of the game.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub application_id: Option<ApplicationId>,
    /// What the user is currently doing.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
    /// The current party status of the user.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// The emoji used for a custom status.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub emoji: Option<ActivityEmoji>,
    /// The current party of the user.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub party: Option<ActivityParty>,
    /// Images and accompanying hover texts for the activity.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub assets: Option<ActivityAssets>,
    /// The secrets for joining or spectating the activity.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secrets: Option<ActivitySecrets>,
    /// Whether the activity is an instanced game session.
    #[serde(default, skip_serializing_if = "is_false")]
    pub instance: bool,
    /// A set of flags describing the activity.
    #[serde(default, skip_serializing_if = "ActivityFlags::is_empty")]
    pub flags: ActivityFlags,
}

impl_eq_fields!(ActivityTimestamps: [start, end]);
impl_eq_fields!(ActivityEmoji: [name, id, animated]);
impl_eq_fields!(ActivityParty: [id, size]);
impl_eq_fields!(ActivityAssets: [large_image, large_text, small_image, small_text]);
impl_eq_fields!(ActivitySecrets: [join, spectate, game]);
impl_eq_fields!(Activity: [
    name,
    kind,
    url,
    timestamps,
    application_id,
    details,
    state,
    emoji,
    party,
    assets,
    secrets,
    instance,
    flags,
]);

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;
    use chrono::TimeZone;

    #[test]
    fn test_deserialize_activity() {
        let value = json!({
          "details": "24H RL Stream for Charity",
          "state": "Rocket League",
          "name": "Twitch",
          "type": 1,
          "url": "https://www.twitch.tv/discordapp"
        });
        let activity = Activity {
            name: "Twitch".to_owned(),
            kind: ActivityType::Streaming,
            url: Some("https://www.twitch.tv/discordapp".to_owned()),
            timestamps: None,
            application_id: None,
            details: Some("24H RL Stream for Charity".to_owned()),
            state: Some("Rocket League".to_owned()),
            emoji: None,
            party: None,
            assets: None,
            secrets: None,
            instance: false,
            flags: ActivityFlags::default(),
        };

        let deserialized = Activity::deserialize(&value).unwrap();

        assert_eq_fields!(activity, deserialized);
    }

    #[test]
    fn test_serialize_activity() {
        let value = json!({
          "details": "24H RL Stream for Charity",
          "state": "Rocket League",
          "name": "Twitch",
          "type": 1,
          "url": "https://www.twitch.tv/discordapp"
        });
        let activity = Activity {
            name: "Twitch".to_owned(),
            kind: ActivityType::Streaming,
            url: Some("https://www.twitch.tv/discordapp".to_owned()),
            timestamps: None,
            application_id: None,
            details: Some("24H RL Stream for Charity".to_owned()),
            state: Some("Rocket League".to_owned()),
            emoji: None,
            party: None,
            assets: None,
            secrets: None,
            instance: false,
            flags: ActivityFlags::default(),
        };

        assert_eq!(value, serde_json::to_value(&activity).unwrap());
    }

    #[test]
    fn test_deserialize_activity_rich() {
        let value = json!({
          "name": "Rocket League",
          "type": 0,
          "application_id": "379286085710381999",
          "state": "In a Match",
          "details": "Ranked Duos: 2-1",
          "timestamps": {
            "start": 15112000660000i64
          },
          "party": {
            "id": "9dd6594e-81b3-49f6-a6b5-a679e6a060d3",
            "size": [2, 2]
          },
          "assets": {
            "large_image": "351371005538729000",
            "large_text": "DFH Stadium",
            "small_image": "351371005538729111",
            "small_text": "Silver III"
          },
          "secrets": {
            "join": "025ed05c71f639de8bfaa0d679d7c94b2fdce12f",
            "spectate": "e7eb30d2ee025ed05c71ea495f770b76454ee4e0",
            "match": "4b2fdce12f639de8bfa7e3591b71a0d679d7c93f"
          }
        });
        let activity = Activity {
            name: "Rocket League".to_owned(),
            kind: ActivityType::Game,
            url: None,
            timestamps: Some(ActivityTimestamps {
                start: Some(Utc.timestamp_millis(15112000660000)),
                end: None,
            }),
            application_id: Some(ApplicationId::from(379286085710381999)),
            details: Some("Ranked Duos: 2-1".to_owned()),
            state: Some("In a Match".to_owned()),
            emoji: None,
            party: Some(ActivityParty {
                id: Some("9dd6594e-81b3-49f6-a6b5-a679e6a060d3".to_owned()),
                size: Some([2, 2]),
            }),
            assets: Some(ActivityAssets {
                large_image: Some("351371005538729000".to_owned()),
                large_text: Some("DFH Stadium".to_owned()),
                small_image: Some("351371005538729111".to_owned()),
                small_text: Some("Silver III".to_owned()),
            }),
            secrets: Some(ActivitySecrets {
                join: Some("025ed05c71f639de8bfaa0d679d7c94b2fdce12f".to_owned()),
                spectate: Some("e7eb30d2ee025ed05c71ea495f770b76454ee4e0".to_owned()),
                game: Some("4b2fdce12f639de8bfa7e3591b71a0d679d7c93f".to_owned()),
            }),
            instance: false,
            flags: ActivityFlags::default(),
        };

        let deserialized = Activity::deserialize(&value).unwrap();

        assert_eq_fields!(activity, deserialized);
    }

    #[test]
    fn test_serialize_activity_rich() {
        let value = json!({
          "name": "Rocket League",
          "type": 0,
          "application_id": "379286085710381999",
          "state": "In a Match",
          "details": "Ranked Duos: 2-1",
          "timestamps": {
            "start": 15112000660000i64
          },
          "party": {
            "id": "9dd6594e-81b3-49f6-a6b5-a679e6a060d3",
            "size": [2, 2]
          },
          "assets": {
            "large_image": "351371005538729000",
            "large_text": "DFH Stadium",
            "small_image": "351371005538729111",
            "small_text": "Silver III"
          },
          "secrets": {
            "join": "025ed05c71f639de8bfaa0d679d7c94b2fdce12f",
            "spectate": "e7eb30d2ee025ed05c71ea495f770b76454ee4e0",
            "match": "4b2fdce12f639de8bfa7e3591b71a0d679d7c93f"
          }
        });
        let activity = Activity {
            name: "Rocket League".to_owned(),
            kind: ActivityType::Game,
            url: None,
            timestamps: Some(ActivityTimestamps {
                start: Some(Utc.timestamp_millis(15112000660000)),
                end: None,
            }),
            application_id: Some(ApplicationId::from(379286085710381999)),
            details: Some("Ranked Duos: 2-1".to_owned()),
            state: Some("In a Match".to_owned()),
            emoji: None,
            party: Some(ActivityParty {
                id: Some("9dd6594e-81b3-49f6-a6b5-a679e6a060d3".to_owned()),
                size: Some([2, 2]),
            }),
            assets: Some(ActivityAssets {
                large_image: Some("351371005538729000".to_owned()),
                large_text: Some("DFH Stadium".to_owned()),
                small_image: Some("351371005538729111".to_owned()),
                small_text: Some("Silver III".to_owned()),
            }),
            secrets: Some(ActivitySecrets {
                join: Some("025ed05c71f639de8bfaa0d679d7c94b2fdce12f".to_owned()),
                spectate: Some("e7eb30d2ee025ed05c71ea495f770b76454ee4e0".to_owned()),
                game: Some("4b2fdce12f639de8bfa7e3591b71a0d679d7c93f".to_owned()),
            }),
            instance: false,
            flags: ActivityFlags::default(),
        };

        assert_eq!(value, serde_json::to_value(&activity).unwrap());
    }
}
