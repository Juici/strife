use serde::Serialize;

use crate::model::id::ChannelId;
use crate::model::image::ImageDataRef;

/// A builder for editing a webhook.
#[derive(Debug, Serialize)]
pub struct EditWebhook<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    avatar: Option<&'a ImageDataRef>,
    #[serde(skip_serializing_if = "Option::is_none")]
    channel_id: Option<ChannelId>,
}

impl<'a> EditWebhook<'a> {
    pub(crate) fn new() -> Self {
        EditWebhook {
            name: None,
            avatar: None,
            channel_id: None,
        }
    }

    /// Sets the default name of the webhook.
    pub fn name(&mut self, name: &'a str) {
        self.name = Some(name);
    }

    /// Sets the default avatar of the webhook.
    pub fn avatar(&mut self, avatar: &'a ImageDataRef) {
        self.avatar = Some(avatar);
    }

    /// Sets the channel the webhook should be moved to.
    pub fn channel(&mut self, channel_id: ChannelId) {
        self.channel_id = Some(channel_id);
    }
}
