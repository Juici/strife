//! Builders for making or editing models.

mod create_channel;
mod create_guild;
mod edit_channel;

pub use self::create_channel::CreateChannel;
pub use self::create_guild::CreateGuild;
pub use self::edit_channel::EditChannel;

pub(crate) mod marker {
    pub use super::edit_channel::GuildChannelBuilder;
}
