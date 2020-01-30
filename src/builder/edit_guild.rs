use serde::Serialize;

use crate::model::guild::settings::{
    ExplicitContentFilterLevel, MessageNotificationLevel, VerificationLevel,
};
use crate::model::id::{ChannelId, UserId};
use crate::model::image::ImageDataRef;
use crate::model::voice::VoiceRegionId;

/// A builder for editing a guild.
#[derive(Debug, Serialize)]
pub struct EditGuild<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<VoiceRegionId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    verification_level: Option<VerificationLevel>,
    #[serde(rename = "default_message_notifications")]
    #[serde(skip_serializing_if = "Option::is_none")]
    message_notifications: Option<MessageNotificationLevel>,
    #[serde(rename = "explicit_content_filter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    content_filter: Option<ExplicitContentFilterLevel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    afk_channel_id: Option<ChannelId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    afk_timeout: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    icon: Option<&'a ImageDataRef>,
    #[serde(skip_serializing_if = "Option::is_none")]
    owner_id: Option<UserId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    splash: Option<&'a ImageDataRef>,
    #[serde(skip_serializing_if = "Option::is_none")]
    banner: Option<&'a ImageDataRef>,
    #[serde(skip_serializing_if = "Option::is_none")]
    system_channel_id: Option<ChannelId>,
}

impl<'a> EditGuild<'a> {
    pub(crate) fn new() -> Self {
        EditGuild {
            name: None,
            region: None,
            verification_level: None,
            message_notifications: None,
            content_filter: None,
            afk_channel_id: None,
            afk_timeout: None,
            icon: None,
            owner_id: None,
            splash: None,
            banner: None,
            system_channel_id: None,
        }
    }

    /// Sets the name of the guild.
    pub fn name(&mut self, name: &'a str) {
        self.name = Some(name);
    }

    /// Sets the voice region for the guild.
    pub fn region(&mut self, region: VoiceRegionId) {
        self.region = Some(region);
    }

    /// Sets the required verification level for the guild.
    pub fn verification_level(&mut self, level: VerificationLevel) {
        self.verification_level = Some(level);
    }

    /// Sets the default message notification level for the guild.
    pub fn message_notification_level(&mut self, level: MessageNotificationLevel) {
        self.message_notifications = Some(level);
    }

    /// Sets the default level of filter for explicit content in the guild.
    pub fn content_filter_level(&mut self, level: ExplicitContentFilterLevel) {
        self.content_filter = Some(level);
    }

    /// Sets the AFK voice channel for the guild.
    pub fn afk_channel(&mut self, channel_id: ChannelId) {
        self.afk_channel_id = Some(channel_id);
    }

    /// Sets the AFK timeout in seconds for the guild.
    pub fn afk_timeout(&mut self, afk_timeout: u64) {
        self.afk_timeout = Some(afk_timeout);
    }

    /// Sets the icon for the guild.
    pub fn icon(&mut self, icon: &'a ImageDataRef) {
        self.icon = Some(icon);
    }

    /// Sets the user to transfer guild ownership to.
    ///
    /// **Must be the owner of the guild.**
    pub fn transfer_ownership(&mut self, new_owner_id: UserId) {
        self.owner_id = Some(new_owner_id);
    }

    /// Sets the splash image for the guild.
    ///
    /// Require the guild to have the [`InviteSplash`] feature.
    ///
    /// # Notes
    ///
    /// The image must be 16:9 in PNG or JPEG format.
    #[doc = "\n[`InviteSplash`]: ../model/guild/enum.GuildFeature.html#variant.InviteSplash"]
    pub fn splash(&mut self, splash: &'a ImageDataRef) {
        self.splash = Some(splash);
    }

    /// Sets the banner image for the guild.
    ///
    /// Require the guild to have the [`Banner`] feature.
    ///
    /// # Notes
    ///
    /// The image must be 16:9 in PNG or JPEG format.
    #[doc = "\n[`Banner`]: ../model/guild/enum.GuildFeature.html#variant.Banner"]
    pub fn banner(&mut self, banner: &'a ImageDataRef) {
        self.banner = Some(banner);
    }

    /// Sets the channel in the guild to which system message are sent.
    pub fn system_channel(&mut self, channel_id: ChannelId) {
        self.system_channel_id = Some(channel_id);
    }
}
