//! Builders for making or editing models.

mod create_channel;

pub use self::create_channel::CreateChannel;

pub(crate) mod marker {
    pub use super::create_channel::GuildChannelMarker;
}
