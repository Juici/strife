//! Models related to channels.

// Internal utility module.
mod utils;

mod converse;
mod dm_channel;
mod group;

pub mod guild;
pub mod message;
pub mod permissions;

use async_trait::async_trait;
use serde::de;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::model::id::ChannelId;

pub use self::converse::Converse;
pub use self::dm_channel::DMChannel;
pub use self::group::Group;
pub use self::guild::GuildChannel;
pub use self::message::Message;

/// The type of a channel.
#[non_exhaustive]
#[int_enum::int_enum(u8)]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ChannelType {
    /// A text channel in a guild.
    Text = 0,
    /// A direct message channel between the client user and another user.
    Private = 1,
    /// A voice channel in a guild.
    Voice = 2,
    /// A group message channel between multiple users.
    Group = 3,
    /// An organizational category that contains non-category channels.
    Category = 4,
    /// A channel that users can follow and crosspost into another guild.
    News = 5,
    /// A channel in which game developers can sell games on Discord.
    Store = 6,
}

impl ChannelType {
    pub(crate) fn from_map<E>(
        map: &serde_json::Map<String, serde_json::Value>,
    ) -> Result<ChannelType, E>
    where
        E: de::Error,
    {
        match map.get("type") {
            Some(kind) => match Deserialize::deserialize(kind) {
                Ok(kind) => Ok(kind),
                Err(_) => return Err(E::custom(format_args!("unknown channel type: {}", kind))),
            },
            None => return Err(E::missing_field("type")),
        }
    }
}

/// A channel in Discord.
#[non_exhaustive]
#[remain::sorted]
#[derive(Clone, Debug)]
pub enum Channel {
    /// A direct message channel between the [`ClientUser`] and another
    /// [`User`].
    ///
    /// [`ClientUser`]: ../user/struct.ClientUser.html
    /// [`User`]: ../user/struct.User.html
    DM(DMChannel),
    /// A group message channel between multiple [`User`]s.
    ///
    /// [`User`]: ../user/struct.User.html
    Group(Group),
    /// A channel within a [`Guild`].
    ///
    /// [`Guild`]: TODO
    Guild(GuildChannel),
}

impl Channel {
    fn inner(&self) -> &dyn Converse {
        match self {
            Channel::DM(channel) => channel,
            Channel::Group(channel) => channel,
            Channel::Guild(channel) => channel,
        }
    }
}

#[async_trait]
impl Converse for Channel {
    async fn channel_id(&self) -> ChannelId {
        self.inner().channel_id().await
    }

    fn channel_type(&self) -> ChannelType {
        self.inner().channel_type()
    }
}

impl Serialize for Channel {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Channel::DM(channel) => channel.serialize(serializer),
            Channel::Group(channel) => channel.serialize(serializer),
            Channel::Guild(channel) => channel.serialize(serializer),
        }
    }
}

impl<'de> Deserialize<'de> for Channel {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde_json::{Map, Value};

        // TODO: Run benchmarks and maybe write a custom approach if warranted.

        let map: Map<String, Value> = Map::deserialize(deserializer)?;
        let kind = ChannelType::from_map(&map)?;

        let value = Value::Object(map);
        match kind {
            ChannelType::Text
            | ChannelType::Voice
            | ChannelType::Category
            | ChannelType::News
            | ChannelType::Store => GuildChannel::from_value(kind, value).map(Channel::Guild),
            ChannelType::Private => DMChannel::deserialize(value)
                .map(Channel::DM)
                .map_err(de::Error::custom),
            ChannelType::Group => Group::deserialize(value)
                .map(Channel::Group)
                .map_err(de::Error::custom),
        }
    }
}

impl_eq_fields!(Channel: (a, b) => {
    match (a, b) {
        (Channel::DM(a), Channel::DM(b)) => assert_eq_fields!(a, b),
        (Channel::Group(a), Channel::Group(b)) => assert_eq_fields!(a, b),
        (Channel::Guild(a), Channel::Guild(b)) => assert_eq_fields!(a, b),
        (a, b) => panic_ne_fields!(a, b),
    }
});
