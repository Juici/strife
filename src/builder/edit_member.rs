use serde::Serialize;

use crate::model::id::{ChannelId, RoleId};

/// A builder for editing a guild member.
#[derive(Debug, Serialize)]
pub struct EditMember<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    nick: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    roles: Option<&'a [RoleId]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mute: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deaf: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    channel_id: Option<Option<ChannelId>>,
}

impl<'a> EditMember<'a> {
    pub(crate) fn new() -> Self {
        Self {
            nick: None,
            roles: None,
            mute: None,
            deaf: None,
            channel_id: None,
        }
    }

    /// Sets the nickname of the member.
    ///
    /// Requires the [`MANAGE_NICKNAMES`] permission.
    #[doc = "\n[`MANAGE_NICKNAMES`]: ../model/permissions/struct.Permissions.html#associatedconstant.MANAGE_NICKNAMES"]
    pub fn nick(&mut self, nick: &'a str) {
        self.nick = Some(nick);
    }

    /// Sets the roles of the member.
    ///
    /// Requires the [`MANAGE_ROLES`] permission.
    #[doc = "\n[`MANAGE_ROLES`]: ../model/permissions/struct.Permissions.html#associatedconstant.MANAGE_ROLES"]
    pub fn roles(&mut self, roles: &'a [RoleId]) {
        self.roles = Some(roles);
    }

    /// Sets whether the member is muted.
    ///
    /// Requires the [`MUTE_MEMBERS`] permission.
    #[doc = "\n[`MUTE_MEMBERS`]: ../model/permissions/struct.Permissions.html#associatedconstant.MUTE_MEMBERS"]
    pub fn muted(&mut self, muted: bool) {
        self.mute = Some(muted);
    }

    /// Sets whether the member is deafened.
    ///
    /// Requires the [`DEAFEN_MEMBERS`] permission.
    #[doc = "\n[`DEAFEN_MEMBERS`]: ../model/permissions/struct.Permissions.html#associatedconstant.DEAFEN_MEMBERS"]
    pub fn deafened(&mut self, deafened: bool) {
        self.deaf = Some(deafened);
    }

    /// Moves the member to the specified voice channel.
    ///
    /// Requires the [`MOVE_MEMBERS`] permission.
    #[doc = "\n[`MOVE_MEMBERS`]: ../model/permissions/struct.Permissions.html#associatedconstant.MOVE_MEMBERS"]
    pub fn move_channel(&mut self, channel_id: ChannelId) {
        self.channel_id = Some(Some(channel_id));
    }

    /// Disconnects the member from their current voice channel.
    ///
    /// Requires the [`MOVE_MEMBERS`] permission.
    #[doc = "\n[`MOVE_MEMBERS`]: ../model/permissions/struct.Permissions.html#associatedconstant.MOVE_MEMBERS"]
    pub fn disconnect(&mut self) {
        self.channel_id = Some(None);
    }
}
