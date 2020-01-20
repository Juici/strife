use std::ops::{Deref, DerefMut};

use serde::Serialize;

use crate::model::channel::guild::{
    Category, NewsChannel, StoreChannel, TextChannel, VoiceChannel,
};
use crate::model::channel::permissions::PermissionOverwrite;
use crate::model::channel::ChannelType;
use crate::model::id::{ChannelId, ToSnowflakeId};

mod private {
    pub trait Sealed {}
}

pub trait GuildChannelMarker: private::Sealed {
    type Container: Serialize + Default;

    fn kind() -> ChannelType;
}

macro_rules! guild_channel {
    ($($channel:ident = [$container:ident, $channel_type:path]);* $(;)?) => {$(
        #[doc(hidden)]
        impl private::Sealed for $channel {}

        #[doc(hidden)]
        impl GuildChannelMarker for $channel {
            type Container = $container;

            #[inline]
            fn kind() -> ChannelType {
                $channel_type
            }
        }
    )*};
}

guild_channel! {
    Category = [CreateCategory, ChannelType::Category];
    NewsChannel = [CreateNewsChannel, ChannelType::News];
    StoreChannel = [CreateStoreChannel, ChannelType::Store];
    TextChannel = [CreateTextChannel, ChannelType::Text];
    VoiceChannel = [CreateVoiceChannel, ChannelType::Voice];
}

/// A builder for creating a new channel in a guild.
#[derive(Serialize)]
pub struct CreateChannel<T: GuildChannelMarker> {
    #[serde(flatten)]
    container: T::Container,
    #[serde(rename = "type")]
    kind: ChannelType,
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    position: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    permission_overwrites: Option<Vec<PermissionOverwrite>>,
}

impl<T: GuildChannelMarker> CreateChannel<T> {
    pub(crate) fn create<S: Into<String>>(name: S) -> CreateChannel<T> {
        CreateChannel {
            container: T::Container::default(),
            kind: T::kind(),
            name: name.into(),
            position: None,
            permission_overwrites: None,
        }
    }

    /// Sets the position of the channel.
    pub fn position(&mut self, position: usize) {
        self.position = Some(position);
    }

    /// Sets the permission overwrites for the channel.
    pub fn permission_overwrites<V>(&mut self, permissions: V)
    where
        V: Into<Vec<PermissionOverwrite>>,
    {
        self.permission_overwrites = Some(permissions.into());
    }
}

impl<T: GuildChannelMarker> Deref for CreateChannel<T> {
    type Target = T::Container;

    fn deref(&self) -> &Self::Target {
        &self.container
    }
}

impl<T: GuildChannelMarker> DerefMut for CreateChannel<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.container
    }
}

#[derive(Debug, Default, Serialize)]
pub struct CreateCategory {}

#[derive(Debug, Default, Serialize)]
pub struct CreateNewsChannel {
    #[serde(skip_serializing_if = "Option::is_none")]
    topic: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    nsfw: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parent_id: Option<ChannelId>,
}

impl CreateNewsChannel {
    /// Sets the topic of the channel.
    pub fn topic<S>(&mut self, topic: S)
    where
        S: Into<String>,
    {
        self.topic = Some(topic.into());
    }

    /// Sets whether the channel is NSFW.
    pub fn nsfw<S>(&mut self, nsfw: bool) {
        self.nsfw = Some(nsfw);
    }

    /// Sets the parent category of the channel.
    pub fn category<C>(&mut self, category: C)
    where
        C: ToSnowflakeId<Id = ChannelId>,
    {
        self.parent_id = Some(category.id());
    }
}

#[derive(Debug, Default, Serialize)]
pub struct CreateStoreChannel {
    #[serde(skip_serializing_if = "Option::is_none")]
    nsfw: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parent_id: Option<ChannelId>,
}

impl CreateStoreChannel {
    /// Sets whether the channel is NSFW.
    pub fn nsfw<S>(&mut self, nsfw: bool) {
        self.nsfw = Some(nsfw);
    }

    /// Sets the parent category of the channel.
    pub fn category<C>(&mut self, category: C)
    where
        C: ToSnowflakeId<Id = ChannelId>,
    {
        self.parent_id = Some(category.id());
    }
}

#[derive(Debug, Default, Serialize)]
pub struct CreateTextChannel {
    #[serde(skip_serializing_if = "Option::is_none")]
    topic: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    nsfw: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rate_limit_per_user: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parent_id: Option<ChannelId>,
}

impl CreateTextChannel {
    /// Sets the topic of the channel.
    pub fn topic<S>(&mut self, topic: S)
    where
        S: Into<String>,
    {
        self.topic = Some(topic.into());
    }

    /// Sets whether the channel is NSFW.
    pub fn nsfw<S>(&mut self, nsfw: bool) {
        self.nsfw = Some(nsfw);
    }

    /// Sets message rate limit for the channel.
    pub fn rate_limit(&mut self, rate_limit: u16) {
        self.rate_limit_per_user = Some(rate_limit);
    }

    /// Sets the parent category of the channel.
    pub fn category<C>(&mut self, category: C)
    where
        C: ToSnowflakeId<Id = ChannelId>,
    {
        self.parent_id = Some(category.id());
    }
}

#[derive(Debug, Default, Serialize)]
pub struct CreateVoiceChannel {
    #[serde(skip_serializing_if = "Option::is_none")]
    bitrate: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_limit: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parent_id: Option<ChannelId>,
}

impl CreateVoiceChannel {
    /// Sets the bitrate of the channel.
    pub fn bitrate<S>(&mut self, bitrate: u32) {
        self.bitrate = Some(bitrate);
    }

    /// Sets user limit for the channel.
    pub fn rate_limit(&mut self, user_limit: u8) {
        self.user_limit = Some(user_limit);
    }

    /// Sets the parent category of the channel.
    pub fn category<C>(&mut self, category: C)
    where
        C: ToSnowflakeId<Id = ChannelId>,
    {
        self.parent_id = Some(category.id());
    }
}
