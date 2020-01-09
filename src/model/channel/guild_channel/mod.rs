use serde::de;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::model::channel::ChannelType;

/// A channel in a [`Guild`].
///
/// [`Guild`]: TODO
#[non_exhaustive]
#[derive(Clone, Debug)]
pub enum GuildChannel {
    /// A text channel in a [`Guild`].
    ///
    /// [`Guild`]: TODO
    Text, // TODO: Add TextChannel.
    /// A voice channel in a [`Guild`].
    ///
    /// [`Guild`]: TODO
    Voice, // TODO: Add VoiceChannel.
    /// An organizational category that contains non-category channels.
    Category, // TODO: Add Category.
    /// A channel that users can follow and crosspost into another [`Guild`].
    ///
    /// [`Guild`]: TODO
    News, // TODO: Add NewsChannel.
    /// A channel in which game developers can sell games on Discord.
    Store, // TODO: Add StoreChannel.
}

impl GuildChannel {
    pub(crate) fn from_value<E>(
        kind: ChannelType,
        _value: serde_json::Value,
    ) -> Result<GuildChannel, E>
    where
        E: de::Error,
    {
        match kind {
            ChannelType::Category => todo!(),
            ChannelType::News => todo!(),
            ChannelType::Store => todo!(),
            ChannelType::Text => todo!(),
            ChannelType::Voice => todo!(),
            kind => Err(E::custom(format_args!(
                "invalid channel type for guild channel: {:?}",
                kind
            ))),
        }
    }
}

impl Serialize for GuildChannel {
    fn serialize<S>(&self, _serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            GuildChannel::Category => todo!(),
            GuildChannel::News => todo!(),
            GuildChannel::Store => todo!(),
            GuildChannel::Text => todo!(),
            GuildChannel::Voice => todo!(),
        }
    }
}

impl<'de> Deserialize<'de> for GuildChannel {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde_json::{Map, Value};

        let map: Map<String, Value> = Map::deserialize(deserializer)?;
        let kind = ChannelType::from_map(&map)?;

        let value = Value::Object(map);
        GuildChannel::from_value(kind, value)
    }
}

impl_eq_fields!(GuildChannel: (_a, _b) => { todo!() });
