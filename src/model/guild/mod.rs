//! Models related to guilds.

mod audit_log;
mod emoji;
mod member;
mod role;

use std::borrow::Cow;
use std::collections::{HashMap, HashSet};
use std::fmt::{self, Display};
use std::ops::Deref;

use chrono::{DateTime, FixedOffset};
use num_traits::Zero;
use serde::{Deserialize, Serialize};

use crate::model::channel::GuildChannel;
use crate::model::id::{ApplicationId, ChannelId, EmojiId, GuildId, RoleId, UserId};
use crate::model::permissions::Permissions;
use crate::model::utils::{is_false, serde_id_map};
use crate::model::voice::{VoiceRegionId, VoiceState};

pub use self::audit_log::AuditLogEvent;
pub use self::emoji::{CustomEmoji, Emoji, PartialEmoji};
pub use self::member::{Member, PartialMember};
pub use self::role::Role;
use crate::model::gateway::presence::Presence;

/// The required level of criteria a user must meet, prior to being able to send
/// messages in a [`Guild`].
///
/// [`Guild`]: struct.Guild.html
#[non_exhaustive]
#[int_enum::int_enum(u8)]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum VerificationLevel {
    /// Does not require any verification.
    None = 0,
    /// Must have a verified email on the user's Discord account.
    Low = 1,
    /// Must also be a registered user on Discord for longer than 5 minutes.
    Medium = 2,
    /// Must also be a member of the guild for longer than 10 minutes.
    High = 3,
    /// Must have a verified phone on the user's Discord account.
    Higher = 4,
}

impl Default for VerificationLevel {
    fn default() -> Self {
        VerificationLevel::None
    }
}

/// The default level of message notifications in a guild.
#[non_exhaustive]
#[int_enum::int_enum(u8)]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum MessageNotificationLevel {
    /// All messages will send notifications.
    AllMessages = 0,
    /// Only messages that mention a user or a user's role will send
    /// notifications.
    OnlyMentions = 1,
}

impl Default for MessageNotificationLevel {
    fn default() -> Self {
        MessageNotificationLevel::AllMessages
    }
}

/// The level of filter to apply to users that send messages containing explicit
/// content.
#[non_exhaustive]
#[int_enum::int_enum(u8)]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ExplicitContentFilterLevel {
    /// No filter will be applied.
    Disabled = 0,
    /// Only members with roles will be able to send explicit content.
    MembersWithoutRoles = 1,
    /// All members will have explicit content filtered from messages they send.
    AllMembers = 2,
}

impl Default for ExplicitContentFilterLevel {
    fn default() -> Self {
        ExplicitContentFilterLevel::Disabled
    }
}

/// A feature enabled for a guild.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum GuildFeature {
    /// The guild has access to set an invite splash background.
    InviteSplash,
    /// The guild has to set 384kbps bitrate in voice.
    VipRegions,
    /// The guild has access to set a vanity URL.
    VanityUrl,
    /// The guild is verified.
    Verified,
    /// The guild is partnered.
    Partnered,
    /// The guild is public.
    Public,
    /// The guild has access to use commerce features (i.e. create store
    /// channels).
    Commerce,
    /// The guild has access to create news channels.
    News,
    /// The guild is able to be discovered in the directory.
    Discoverable,
    /// The guild is able to be featured in the directory.
    Featurable,
    /// The guild has access to set an animated guild icon.
    AnimatedIcon,
    /// The guild has access to set a guild banner image.
    Banner,
}

/// The required level of multi-factor authentication required for a guild.
#[non_exhaustive]
#[int_enum::int_enum(u8)]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum MfaLevel {
    /// Multi-factor authentication is not required.
    None = 0,
    /// Multi-factor authentication is required for members to take moderation
    /// actions (eg. kick, ban, delete messages).
    Elevated = 1,
}

impl Default for MfaLevel {
    fn default() -> Self {
        MfaLevel::None
    }
}

/// The tier of premium for a guild, provided by Nitro boosts.
#[non_exhaustive]
#[int_enum::int_enum(u8)]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum PremiumTier {
    /// Not premium.
    None = 0,
    /// Tier 1.
    Tier1 = 1,
    /// Tier 2.
    Tier2 = 2,
    /// Tier 3.
    Tier3 = 3,
}

impl Default for PremiumTier {
    fn default() -> Self {
        PremiumTier::None
    }
}

/// A locale.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Deserialize, Serialize)]
#[serde(transparent)]
pub struct Locale(pub(crate) Cow<'static, str>);

impl Locale {
    /// Locale defaults to `en-US`.
    pub const DEFAULT: Locale = Locale::from_static("en-US");

    /// Creates a voice region ID from a static string.
    pub const fn from_static(id: &'static str) -> Locale {
        Locale(Cow::Borrowed(id))
    }

    /// Creates a voice region ID from a string.
    pub fn from_string<S: Into<String>>(id: S) -> Locale {
        Locale(Cow::Owned(id.into()))
    }
}

impl Default for Locale {
    fn default() -> Self {
        Locale::DEFAULT
    }
}

impl From<&'static str> for Locale {
    fn from(s: &'static str) -> Self {
        Locale(Cow::Borrowed(s))
    }
}

impl From<String> for Locale {
    fn from(s: String) -> Self {
        Locale(Cow::Owned(s))
    }
}

impl Display for Locale {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl Deref for Locale {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

impl AsRef<str> for Locale {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

impl From<Locale> for String {
    fn from(id: Locale) -> Self {
        id.0.to_string()
    }
}

impl From<Locale> for Cow<'static, str> {
    fn from(id: Locale) -> Self {
        id.0
    }
}

impl PartialEq<str> for Locale {
    fn eq(&self, other: &str) -> bool {
        self.0 == other
    }
}

impl PartialEq<String> for Locale {
    fn eq(&self, other: &String) -> bool {
        self.0 == &other[..]
    }
}

impl<'a> PartialEq<Cow<'a, str>> for Locale {
    fn eq(&self, other: &Cow<'a, str>) -> bool {
        &self.0[..] == &other[..]
    }
}

/// A guild with partial information.
#[non_exhaustive]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PartialGuild {
    /// The ID of the guild.
    pub id: GuildId,
    /// The name of the guild.
    pub name: String,
    /// The hash of the guild icon.
    pub icon: Option<String>,
    /// Whether the client user is the owner of the guild.
    #[serde(default, skip_serializing_if = "is_false")]
    pub owner: bool,
    /// The set of permissions for the client user in the guild (excluding
    /// channel permission overwrites).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Permissions>,
}

/// A guild in Discord represents an isolated collection of users and channels,
/// and are often referred to as "servers" in the UI.
#[non_exhaustive]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Guild {
    #[serde(flatten)]
    guild: PartialGuild,
    /// The hash of the guild splash.
    pub splash: Option<String>,
    /// The ID of the owner of the guild.
    pub owner_id: UserId,
    /// The ID of the guild voice region.
    pub region: VoiceRegionId,
    /// The ID of the AFK channel.
    pub afk_channel_id: Option<ChannelId>,
    /// The AFK timeout in seconds.
    pub afk_timeout: u64,
    /// Whether the guild is embeddable (eg. widget).
    #[serde(default, skip_serializing_if = "is_false")]
    pub embed_enabled: bool,
    /// The ID of the channel that the embed widget will generate in invite to.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub embed_channel_id: Option<ChannelId>,
    /// The verification level required for the guild.
    pub verification_level: VerificationLevel,
    /// The default message notification level in the guild.
    #[serde(rename = "default_message_notifications")]
    pub message_notifications: MessageNotificationLevel,
    /// The level at which explicit content will be filtered.
    pub explicit_content_filter: ExplicitContentFilterLevel,
    /// The roles in the guild.
    #[serde(with = "serde_id_map")]
    pub roles: HashMap<RoleId, Role>,
    /// The roles in the guild.
    #[serde(with = "serde_id_map")]
    pub emojis: HashMap<EmojiId, Emoji>,
    /// The features enabled for the guild.
    pub features: HashSet<GuildFeature>,
    /// The required MFA level for the guild.
    pub mfa_level: MfaLevel,
    /// The application ID of the guild creator, if the guild is bot-created.
    pub application_id: Option<ApplicationId>,
    /// Whether the server widget is enabled.
    #[serde(default, skip_serializing_if = "is_false")]
    pub widget_enabled: bool,
    /// The ID of the channel for the server widget.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub widget_channel_id: Option<ChannelId>,
    /// The ID of the channel to which system messages are sent.
    pub system_channel_id: Option<ChannelId>,
    /// When the guild was joined.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub joined_at: Option<DateTime<FixedOffset>>,
    /// Whether the guild is considered a large guild.
    #[serde(default, skip_serializing_if = "is_false")]
    pub large: bool,
    /// Whether the guild is unavailable.
    #[serde(default, skip_serializing_if = "is_false")]
    pub unavailable: bool,
    /// The total number of members in the guild.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub member_count: Option<u64>,
    /// The states of all current voice connections in the guild.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub voice_states: Vec<VoiceState>,
    /// The users in the guild.
    #[serde(with = "serde_id_map")]
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub members: HashMap<UserId, Member>,
    /// The channels in the guild.
    #[serde(with = "serde_id_map")]
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub channels: HashMap<ChannelId, GuildChannel>,
    /// The presences of the users in the guild.
    #[serde(with = "serde_id_map")]
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub presences: HashMap<UserId, Presence>,
    /// The maximum amount of members for the guild.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_members: Option<u64>,
    /// The vanity URL of the guild.
    pub vanity_url_code: Option<String>,
    /// The description of the guild.
    pub description: Option<String>,
    /// The hash of the guild banner.
    pub banner: Option<String>,
    /// The tier of premium for the guild, provided by Nitro boosts.
    #[serde(default)]
    pub premium_tier: PremiumTier,
    /// The total number of users currently boosting the guild.
    #[serde(rename = "premium_subscription_count")]
    #[serde(default, skip_serializing_if = "Zero::is_zero")]
    pub boost_count: u64,
    /// The preferred locale of the guild.
    ///
    /// Defaults to `en-US` if the guild does not have the [`Discoverable`]
    /// feature enabled.
    ///
    /// [`Discoverable`]: enum.GuildFeature.html#variant.Discoverable
    #[serde(default)]
    pub preferred_locale: Locale,
}
wrap!(Guild => mut guild: PartialGuild);

impl_eq_fields!(PartialGuild: [id, name, icon, owner, permissions]);
impl_eq_fields!(Guild: (a, b) => {
    assert_eq_fields!(a, b, [
        guild,
        splash,
        owner_id,
        region,
        afk_channel_id,
        afk_timeout,
        embed_enabled,
        embed_channel_id,
        verification_level,
        message_notifications,
        explicit_content_filter,
        features,
        mfa_level,
        application_id,
        widget_enabled,
        widget_channel_id,
        system_channel_id,
        joined_at,
        large,
        unavailable,
        member_count,
        voice_states,
        max_members,
        vanity_url_code,
        description,
        banner,
        premium_tier,
        boost_count,
        preferred_locale,
    ]);

    assert_eq_fields!(map => a.roles, b.roles);
    assert_eq_fields!(map => a.emojis, b.emojis);
    assert_eq_fields!(map => a.members, b.members);
    assert_eq_fields!(map => a.channels, b.channels);
    assert_eq_fields!(map => a.presences, b.presences);
});

#[cfg(test)]
mod tests {
    use std::iter::FromIterator;

    use serde_json::json;

    use super::*;

    #[test]
    fn test_deserialize_guild() {
        let value = json!({
          "id": "41771983423143937",
          "application_id": null,
          "name": "Discord Developers",
          "icon": "86e39f7ae3307e811784e2ffd11a7310",
          "splash": null,
          "owner_id": "80351110224678912",
          "region": "us-east",
          "afk_channel_id": "42072017402331136",
          "afk_timeout": 300,
          "embed_enabled": true,
          "embed_channel_id": "41771983444115456",
          "verification_level": 1,
          "default_message_notifications": 0,
          "explicit_content_filter": 0,
          "mfa_level": 0,
          "widget_enabled": false,
          "widget_channel_id": "41771983423143937",
          "roles": [],
          "emojis": [],
          "features": ["INVITE_SPLASH"],
          "unavailable": false
        });
        let guild = Guild {
            guild: PartialGuild {
                id: GuildId::from(41771983423143937),
                name: "Discord Developers".to_owned(),
                icon: Some("86e39f7ae3307e811784e2ffd11a7310".to_owned()),
                owner: false,
                permissions: None,
            },
            splash: None,
            owner_id: UserId::from(80351110224678912),
            region: VoiceRegionId::US_EAST,
            afk_channel_id: Some(ChannelId::from(42072017402331136)),
            afk_timeout: 300,
            embed_enabled: true,
            embed_channel_id: Some(ChannelId::from(41771983444115456)),
            verification_level: VerificationLevel::Low,
            message_notifications: MessageNotificationLevel::AllMessages,
            explicit_content_filter: ExplicitContentFilterLevel::Disabled,
            roles: HashMap::default(),
            emojis: HashMap::default(),
            features: HashSet::from_iter(vec![GuildFeature::InviteSplash]),
            mfa_level: MfaLevel::None,
            application_id: None,
            widget_enabled: false,
            widget_channel_id: Some(ChannelId::from(41771983423143937)),
            system_channel_id: None,
            joined_at: None,
            large: false,
            unavailable: false,
            member_count: None,
            voice_states: vec![],
            members: HashMap::default(),
            channels: HashMap::default(),
            presences: HashMap::default(),
            max_members: None,
            vanity_url_code: None,
            description: None,
            banner: None,
            premium_tier: PremiumTier::None,
            boost_count: 0,
            preferred_locale: Locale::DEFAULT,
        };

        let deserialized = Guild::deserialize(&value).unwrap();
        assert_eq_fields!(guild, deserialized);
    }
}
