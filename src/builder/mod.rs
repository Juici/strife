//! Builders for making or editing models.

mod create_channel;
mod create_embed;
mod create_guild;
mod create_invite;
mod create_message;
mod edit_role;

mod edit_channel;
mod edit_current_user;
mod edit_guild;
mod edit_guild_embed;
mod edit_member;
mod edit_message;

pub use self::create_channel::CreateChannel;
pub use self::create_embed::CreateEmbed;
pub use self::create_guild::CreateGuild;
pub use self::create_invite::CreateInvite;
pub use self::create_message::CreateMessage;

pub use self::edit_channel::EditChannel;
pub use self::edit_current_user::EditCurrentUser;
pub use self::edit_guild::EditGuild;
pub use self::edit_guild_embed::EditGuildEmbed;
pub use self::edit_member::EditMember;
pub use self::edit_message::EditMessage;
pub use self::edit_role::EditRole;

pub(crate) mod marker {
    pub use super::edit_channel::GuildChannelBuilder;
}
