use serde::Serialize;

use crate::builder::CreateEmbed;

// TODO: Look into `nonce`.
// TODO: Add support for files.

/// A builder for creating a new message.
#[derive(Debug, Serialize)]
pub struct CreateMessage<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    content: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tts: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    embed: Option<CreateEmbed<'a>>,
}

impl<'a> CreateMessage<'a> {
    pub(crate) fn new() -> Self {
        CreateMessage {
            content: None,
            tts: None,
            embed: None,
        }
    }

    /// Sets the content of the message (maximum 2000 characters).
    pub fn content(&mut self, content: &'a str) {
        self.content = Some(content);
    }

    /// Sets whether the message is a TTS message.
    ///
    /// Requires the [`SEND_TTS_MESSAGES`] permission if set to `true`.
    #[doc = "\n[`SEND_TTS_MESSAGES`]: ../model/permissions/struct.Permissions.html#associatedconstant.SEND_TTS_MESSAGES"]
    pub fn tts(&mut self, tts: bool) {
        self.tts = Some(tts);
    }

    /// Sets the embedded rich content of the message.
    pub fn embed<F>(&mut self, create_embed: F)
    where
        F: FnOnce(&mut CreateEmbed),
    {
        let mut embed = CreateEmbed::new();
        create_embed(&mut embed);

        self.embed = Some(embed);
    }
}
