//! Guild channel models.

mod category;
mod news_channel;
mod store_channel;
mod text_channel;
mod voice_channel;

use async_trait::async_trait;
use serde::de;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::model::channel::{ChannelType, Converse};

pub use self::category::Category;
pub use self::news_channel::NewsChannel;
pub use self::store_channel::StoreChannel;
pub use self::text_channel::TextChannel;
pub use self::voice_channel::VoiceChannel;
use crate::model::id::ChannelId;

/// A channel in a [`Guild`].
///
/// [`Guild`]: TODO
#[non_exhaustive]
#[derive(Clone, Debug)]
pub enum GuildChannel {
    /// A text channel in a [`Guild`].
    ///
    /// [`Guild`]: TODO
    Text(TextChannel),
    /// A voice channel in a [`Guild`].
    ///
    /// [`Guild`]: TODO
    Voice(VoiceChannel),
    /// An organizational category that contains non-category channels.
    Category(Category),
    /// A channel that users can follow and crosspost into another [`Guild`].
    ///
    /// [`Guild`]: TODO
    News(NewsChannel),
    /// A channel in which game developers can sell games on Discord.
    Store(StoreChannel),
}

impl GuildChannel {
    fn inner(&self) -> &dyn Converse {
        match self {
            GuildChannel::Text(channel) => channel,
            GuildChannel::Voice(channel) => channel,
            GuildChannel::Category(channel) => channel,
            GuildChannel::News(channel) => channel,
            GuildChannel::Store(channel) => channel,
        }
    }
}

#[async_trait]
impl Converse for GuildChannel {
    async fn channel_id(&self) -> ChannelId {
        self.inner().channel_id().await
    }

    fn channel_type(&self) -> ChannelType {
        self.inner().channel_type()
    }
}

impl GuildChannel {
    pub(crate) fn from_value<E>(
        kind: ChannelType,
        value: serde_json::Value,
    ) -> Result<GuildChannel, E>
    where
        E: de::Error,
    {
        let result = match kind {
            ChannelType::Text => TextChannel::deserialize(value).map(GuildChannel::Text),
            ChannelType::Voice => VoiceChannel::deserialize(value).map(GuildChannel::Voice),
            ChannelType::Category => Category::deserialize(value).map(GuildChannel::Category),
            ChannelType::News => NewsChannel::deserialize(value).map(GuildChannel::News),
            ChannelType::Store => StoreChannel::deserialize(value).map(GuildChannel::Store),
            kind => {
                return Err(E::custom(format_args!(
                    "invalid channel type for guild channel: {:?}",
                    kind
                )))
            }
        };
        result.map_err(E::custom)
    }
}

impl Serialize for GuildChannel {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            GuildChannel::Text(channel) => channel.serialize(serializer),
            GuildChannel::Voice(channel) => channel.serialize(serializer),
            GuildChannel::Category(channel) => channel.serialize(serializer),
            GuildChannel::News(channel) => channel.serialize(serializer),
            GuildChannel::Store(channel) => channel.serialize(serializer),
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

impl_eq_fields!(GuildChannel: (a, b) => {
    match (a, b) {
        (GuildChannel::Text(a), GuildChannel::Text(b)) => assert_eq_fields!(a, b),
        (GuildChannel::Voice(a), GuildChannel::Voice(b)) => assert_eq_fields!(a, b),
        (GuildChannel::Category(a), GuildChannel::Category(b)) => assert_eq_fields!(a, b),
        (GuildChannel::News(a), GuildChannel::News(b)) => assert_eq_fields!(a, b),
        (GuildChannel::Store(a), GuildChannel::Store(b)) => assert_eq_fields!(a, b),
        (a, b) => panic_ne_fields!(a, b),
    }
});
