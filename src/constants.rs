//! A collection of constants used by the library.

use bitflags::bitflags;
use int_enum::IntEnum;

/// The gateway version used by the library, URI is retrieved via the REST API.
pub const GATEWAY_VERSION: usize = 6;

/// The maximum length of textual size of an embed message.
pub const EMBED_MAX_LENGTH: usize = 6000;
/// The maximum length of a message in Unicode code points allowed by Discord.
pub const MESSAGE_MAX_LENGTH: usize = 2000;

/// The UserAgent header sent with every request.
pub const USER_AGENT: &str = concat!("DiscordBot (", pkg_repo!(), ", ", pkg_version!(), ")");

/// Gateway opcodes.
#[repr(u8)]
#[non_exhaustive]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, IntEnum)]
pub enum OpCode {
    /// Dispatches an event.
    Event = 0,
    /// Used for ping checking.
    Heartbeat = 1,
    /// Used for client handshake.
    Identify = 2,
    /// Used to update the client status.
    StatusUpdate = 3,
    /// Used to join/move/leave voice channels.
    VoiceStateUpdate = 4,
    /// Used for voice ping checking.
    VoiceServerPing = 5,
    /// Used to resume a closed connection.
    Resume = 6,
    /// Used to tell clients to reconnect to the gateway.
    Reconnect = 7,
    /// Used to request guild members.
    GetGuildMembers = 8,
    /// Used to notify clients that they have an invalid session Id.
    InvalidSession = 9,
    /// Sent immediately after connection, contains heartbeat + server info.
    Hello = 10,
    /// Sent immediately following a client heartbeat that was received.
    HeartbeatAck = 11,
}

/// Gateway close event codes.
pub mod close_codes {
    /// Unknown error. Try reconnecting?
    pub const UNKNOWN_ERROR: u16 = 4000;
    /// An invalid gateway opcode or an invalid payload for an opcode was sent.
    pub const UNKNOWN_OPCODE: u16 = 4001;
    /// An invalid payload was sent.
    pub const DECODE_ERROR: u16 = 4002;
    /// A payload was sent prior to identifying.
    pub const NOT_AUTHENTICATED: u16 = 4003;
    /// The account token sent with your identify payload is incorrect.
    pub const AUTHENTICATION_FAILED: u16 = 4004;
    /// More than one identify payload was sent.
    pub const ALREADY_AUTHENTICATED: u16 = 4005;
    /// The sequence sent when resuming the session was invalid.
    pub const INVALID_SEQUENCE: u16 = 4007;
    /// Payloads were sent too quickly.
    pub const RATE_LIMITED: u16 = 4008;
    /// Session timed out.
    pub const SESSION_TIMEOUT: u16 = 4009;
    /// An invalid shard was sent when identifying.
    pub const INVALID_SHARD: u16 = 4010;
    /// The session would have handled too many guilds.
    pub const SHARDING_REQUIRED: u16 = 4011;
}

bitflags! {
    /// Intents used when identifying to the gateway, to subscribe to events.
    ///
    /// Each intent represents a pre-defined group of events; these events will
    /// not be sent by the gateway unless subscribed to.
    pub struct Intents: u32 {
        /// ```text
        /// - GUILD_CREATE
        /// - GUILD_DELETE
        /// - GUILD_ROLE_CREATE
        /// - GUILD_ROLE_UPDATE
        /// - GUILD_ROLE_DELETE
        /// - CHANNEL_CREATE
        /// - CHANNEL_UPDATE
        /// - CHANNEL_DELETE
        /// - CHANNEL_PINS_UPDATE
        /// ```
        const GUILDS = (1 << 0);

        /// ```text
        /// - GUILD_MEMBER_ADD
        /// - GUILD_MEMBER_UPDATE
        /// - GUILD_MEMBER_REMOVE
        /// ```
        const GUILD_MEMBERS = (1 << 1);

        /// ```text
        /// - GUILD_BAN_ADD
        /// - GUILD_BAN_REMOVE
        /// ```
        const GUILD_BANS = (1 << 2);

        /// ```text
        /// - GUILD_EMOJIS_UPDATE
        /// ```
        const GUILD_EMOJIS = (1 << 3);

        /// ```text
        /// - GUILD_INTEGRATIONS_UPDATE
        /// ```
        const GUILD_INTEGRATIONS = (1 << 4);

        /// ```text
        /// - WEBHOOKS_UPDATE
        /// ```
        const GUILD_WEBHOOKS = (1 << 5);

        /// ```text
        /// - INVITE_CREATE
        /// - INVITE_DELETE
        /// ```
        const GUILD_INVITES = (1 << 6);

        /// ```text
        /// - VOICE_STATE_UPDATE
        /// ```
        const GUILD_VOICE_STATES = (1 << 7);

        /// ```text
        /// - PRESENCE_UPDATE
        /// ```
        const GUILD_PRESENCES = (1 << 8);

        /// ```text
        /// - MESSAGE_CREATE
        /// - MESSAGE_UPDATE
        /// - MESSAGE_DELETE
        /// ```
        const GUILD_MESSAGES = (1 << 9);

        /// ```text
        /// - MESSAGE_REACTION_ADD
        /// - MESSAGE_REACTION_REMOVE
        /// - MESSAGE_REACTION_REMOVE_ALL
        /// - MESSAGE_REACTION_REMOVE_EMOJI
        /// ```
        const GUILD_MESSAGE_REACTIONS = (1 << 10);

        /// ```text
        /// - TYPING_START
        /// ```
        const GUILD_MESSAGE_TYPING = (1 << 11);

        /// ```text
        /// - CHANNEL_CREATE
        /// - MESSAGE_CREATE
        /// - MESSAGE_UPDATE
        /// - MESSAGE_DELETE
        /// - CHANNEL_PINS_UPDATE
        /// ```
        const DIRECT_MESSAGES = (1 << 12);

        /// ```text
        /// - MESSAGE_REACTION_ADD
        /// - MESSAGE_REACTION_REMOVE
        /// - MESSAGE_REACTION_REMOVE_ALL
        /// - MESSAGE_REACTION_REMOVE_EMOJI
        /// ```
        const DIRECT_MESSAGE_REACTIONS = (1 << 13);

        /// ```text
        /// - TYPING_START
        /// ```
        const DIRECT_MESSAGE_TYPING = (1 << 14);
    }
}
