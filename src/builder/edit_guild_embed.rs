use serde::Serialize;

use crate::model::id::ChannelId;

/// A builder for editing the embed of a guild.
#[derive(Debug, Serialize)]
pub struct EditGuildEmbed {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    channel_id: Option<ChannelId>,
}

impl EditGuildEmbed {
    pub(crate) fn new() -> Self {
        Self {
            enabled: None,
            channel_id: None,
        }
    }

    /// Sets whether the embed is enabled.
    pub fn enabled(&mut self, enabled: bool) {
        self.enabled = Some(enabled);
    }

    /// Sets the channel for the embed.
    pub fn channel(&mut self, channel_id: ChannelId) {
        self.channel_id = Some(channel_id);
    }
}
