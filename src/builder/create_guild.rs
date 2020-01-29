use serde::Serialize;

use crate::model::guild::settings::{
    ExplicitContentFilterLevel, MessageNotificationLevel, VerificationLevel,
};
use crate::model::voice::VoiceRegionId;

/// A builder for creating a new guild.
#[derive(Debug, Serialize)]
pub struct CreateGuild<'a> {
    name: &'a str,
    region: VoiceRegionId,
    icon: Option<&'a str>,
    verification_level: VerificationLevel,
    #[serde(rename = "default_message_notifications")]
    message_notifications: MessageNotificationLevel,
    #[serde(rename = "explicit_content_filter")]
    content_filter: ExplicitContentFilterLevel,
}

impl<'a> CreateGuild<'a> {
    pub(crate) fn create(name: &'a str, region: VoiceRegionId) -> Self {
        CreateGuild {
            name,
            region,
            icon: None,
            verification_level: VerificationLevel::default(),
            message_notifications: MessageNotificationLevel::default(),
            content_filter: ExplicitContentFilterLevel::default(),
        }
    }

    /// Sets the icon for the guild.
    pub fn icon(&mut self, icon: &'a str) {
        self.icon = Some(icon);
    }

    /// Sets the required verification level for the guild.
    pub fn verification_level(&mut self, level: VerificationLevel) {
        self.verification_level = level;
    }

    /// Sets the default message notification level for the guild.
    pub fn message_notification_level(&mut self, level: MessageNotificationLevel) {
        self.message_notifications = level;
    }

    /// Sets the default level of filter for explicit content in the guild.
    pub fn content_filter_level(&mut self, level: ExplicitContentFilterLevel) {
        self.content_filter = level;
    }
}
