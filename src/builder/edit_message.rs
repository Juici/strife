use serde::Serialize;

use crate::builder::CreateEmbed;
use crate::model::channel::message::MessageFlags;

/// A builder for creating a new message.
#[derive(Debug, Serialize)]
pub struct EditMessage<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    content: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    embed: Option<CreateEmbed<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    flags: Option<MessageFlags>,
}

impl<'a> EditMessage<'a> {
    pub(crate) fn new() -> Self {
        EditMessage {
            content: None,
            embed: None,
            flags: None,
        }
    }

    /// Sets the content of the message (maximum 2000 characters).
    pub fn content(&mut self, content: &'a str) {
        self.content = Some(content);
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

    /// Sets the flags of the message.
    ///
    /// Requires the [`MANAGE_MESSAGES`] permission if the client user is not
    /// the original author of the message.
    ///
    /// Currently only [`SUPPRESS_EMBEDS`] can be set/unset.
    ///
    /// # Safety
    ///
    /// No checks are made about which flags are set or unset.
    #[doc = "\n[`MANAGE_MESSAGES`]: ../model/permissions/struct.Permissions.html#associatedconstant.MANAGE_MESSAGES"]
    #[doc = "\n[`SUPPRESS_EMBEDS`]: ../model/channel/message/struct.MessageFlags.html#associatedconstant.SUPPRESS_EMBEDS"]
    pub unsafe fn flags(&mut self, flags: MessageFlags) {
        self.flags = Some(flags);
    }

    /// Sets whether embeds in the message should be suppressed.
    ///
    /// Requires the [`MANAGE_MESSAGES`] permission if the client user is not
    /// the original author of the message.
    #[doc = "\n[`MANAGE_MESSAGES`]: ../model/permissions/struct.Permissions.html#associatedconstant.MANAGE_MESSAGES"]
    pub fn suppress_embeds(&mut self, suppress: bool) {
        self.set_flag(MessageFlags::SUPPRESS_EMBEDS, suppress);
    }

    #[inline]
    fn set_flag(&mut self, flag: MessageFlags, set: bool) {
        match &mut self.flags {
            Some(flags) => flags.set(MessageFlags::SUPPRESS_EMBEDS, set),
            flags @ None => *flags = Some(flag),
        }
    }
}
