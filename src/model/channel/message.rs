use serde::{Deserialize, Serialize};

use crate::model::id::{ChannelId, GuildId, MessageId};
use crate::model::user::User;

/// A message sent in a text channel.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Message {
    /// The ID of the message.
    pub id: MessageId,
    /// The ID of the [`Channel`] the message was sent in.
    pub channel_id: ChannelId,
    /// The ID of the [`Guild`] the message was sent in.
    pub guild_id: Option<GuildId>,
    /// The author of the message.
    ///
    /// May not be a valid user, see notes for more information.
    ///
    /// # Notes
    ///
    /// The author object follows the structure of the user object, but is only
    /// a valid user in the case where the message is generated by a user or bot
    /// user. If the message is generated by a webhook, the author object
    /// corresponds to the webhook's id, username, and avatar. You can tell if a
    /// message is generated by a webhook by checking for the webhook_id on the
    /// message object.
    pub author: User,
    //    pub member: Option<PartialMember>,
}
