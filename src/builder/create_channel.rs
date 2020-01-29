use std::ops::{Deref, DerefMut};

use serde::Serialize;

use crate::builder::marker::GuildChannelBuilder;
use crate::model::channel::permissions::PermissionOverwrite;
use crate::model::channel::ChannelType;

/// A builder for creating a new channel in a guild.
#[derive(Debug, Serialize)]
pub struct CreateChannel<T: GuildChannelBuilder> {
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

impl<T: GuildChannelBuilder> CreateChannel<T> {
    pub(crate) fn create<S: Into<String>>(name: S) -> Self {
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

impl<T: GuildChannelBuilder> Deref for CreateChannel<T> {
    type Target = T::Container;

    fn deref(&self) -> &Self::Target {
        &self.container
    }
}

impl<T: GuildChannelBuilder> DerefMut for CreateChannel<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.container
    }
}
