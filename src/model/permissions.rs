//! Models the [bitwise permissions flags] used by Discord to store
//! [permissions].
//!
//! [bitwise permissions flags]: https://discordapp.com/developers/docs/topics/permissions#permissions-bitwise-permission-flags
//! [permissions]: https://discordapp.com/developers/docs/topics/permissions#permissions

#![allow(clippy::unreadable_literal)]

use bitflags::bitflags;
use serde::de;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::model::utils::U64Visitor;

bitflags! {
    /// A set of permissions for a [`Role`] or [`User`].
    #[derive(Default)]
    pub struct Permissions: u64 {
        /// Allows creation of instant invites.
        const CREATE_INSTANT_INVITE = 0x00000001;
        /// Allows kicking members.
        const KICK_MEMBERS = 0x00000002;
        /// Allows banning members.
        const BAN_MEMBERS = 0x00000004;
        /// Allows all permissions and bypasses channel permission overwrites.
        const ADMINISTRATOR = 0x00000008;
        /// Allows management and editing of channels.
        const MANAGE_CHANNELS = 0x00000010;
        /// Allows management and editing of the guild.
        const MANAGE_GUILD = 0x00000020;
        /// Allows for the addition of reactions to messages.
        const ADD_REACTIONS = 0x00000040;
        /// Allows for viewing of audit logs.
        const VIEW_AUDIT_LOG = 0x00000080;
        /// Allows guild members to view a channel, which includes reading messages in text channels.
        const VIEW_CHANNEL = 0x00000400;
        /// Allows for sending messages in a channel.
        const SEND_MESSAGES = 0x00000800;
        /// Allows for sending of /tts messages.
        const SEND_TTS_MESSAGES = 0x00001000;
        /// Allows for deletion of other users messages.
        const MANAGE_MESSAGES = 0x00002000;
        /// Links sent by users with this permission will be auto-embedded.
        const EMBED_LINKS = 0x00004000;
        /// Allows for uploading images and files.
        const ATTACH_FILES = 0x00008000;
        /// Allows for reading of message history.
        const READ_MESSAGE_HISTORY = 0x00010000;
        /// Allows for using the `@everyone` tag to notify all users in a channel,
        /// and the `@here` tag to notify all online users in a channel.
        const MENTION_EVERYONE = 0x00020000;
        /// Allows the usage of custom emojis from other servers.
        const USE_EXTERNAL_EMOJIS = 0x00040000;
        /// Allows for joining of a voice channel.
        const CONNECT = 0x00100000;
        /// Allows for speaking in a voice channel.
        const SPEAK = 0x00200000;
        /// Allows for muting members in a voice channel.
        const MUTE_MEMBERS = 0x00400000;
        /// Allows for deafening of members in a voice channel.
        const DEAFEN_MEMBERS = 0x00800000;
        /// Allows for moving of members between voice channels.
        const MOVE_MEMBERS = 0x01000000;
        /// Allows for using voice-activity-detection in a voice channel.
        const USE_VAD = 0x02000000;
        /// Allows for using priority speaker in a voice channel.
        const PRIORITY_SPEAKER = 0x00000100;
        /// Allows the user to go live.
        const STREAM = 0x00000200;
        /// Allows for modification of own nickname.
        const CHANGE_NICKNAME = 0x04000000;
        /// Allows for modification of other users nicknames.
        const MANAGE_NICKNAMES = 0x08000000;
        /// Allows management and editing of roles.
        const MANAGE_ROLES = 0x10000000;
        /// Allows management and editing of webhooks.
        const MANAGE_WEBHOOKS = 0x20000000;
        /// Allows management and editing of emojis.
        const MANAGE_EMOJIS = 0x40000000;
    }
}

impl Serialize for Permissions {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u64(self.bits())
    }
}

impl<'de> Deserialize<'de> for Permissions {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let bits = deserializer.deserialize_any(U64Visitor)?;
        match Permissions::from_bits(bits) {
            Some(perms) => Ok(perms),
            None => {
                let unknown: u64 = bits & !Permissions::all().bits();
                Err(de::Error::custom(format!(
                    "unknown permissions bits {:b} in {:b}",
                    unknown, bits
                )))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn test_all() {
        assert_eq!(
            Permissions::all(),
            Permissions::from_bits(2146959359).unwrap()
        );
    }

    #[test]
    fn test_default() {
        assert_eq!(Permissions::default(), Permissions::empty());
    }

    #[test]
    fn test_serialize() {
        let value = json!(103877696);
        let perms = Permissions::from_bits(103877696).unwrap();

        assert_eq!(value, serde_json::to_value(&perms).unwrap());
    }

    #[test]
    fn test_deserialize() {
        let value = json!(68608);
        let perms = Permissions::VIEW_CHANNEL
            | Permissions::READ_MESSAGE_HISTORY
            | Permissions::SEND_MESSAGES;

        assert_eq!(perms, serde_json::from_value(value).unwrap());
    }

    #[test]
    fn test_deserialize_invalid() {
        let value = serde_json::json!(0x00080000);
        let err = serde_json::from_value::<Permissions>(value);

        assert!(err.is_err());
        assert!(err.unwrap_err().is_data());
    }
}
