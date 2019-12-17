use serde::{Deserialize, Serialize};

use crate::model::id::{ChannelId, GuildId, MessageId};

/// A message sent in a text channel.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Message {
    /// The ID of the message.
    pub id: MessageId,
    /// The ID of the [`Channel`] the message was sent in.
    pub channel_id: ChannelId,
    /// The ID of the [`Guild`] the message was sent in.
    pub guild_id: Option<GuildId>,
//    pub author: User,
//    pub member: Option<PartialMember>,
}
