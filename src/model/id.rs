//! Strongly-typed snowflake IDs.

use serde::{Deserialize, Serialize};

use crate::model::snowflake::Snowflake;

/// The ID of an [`Application`].
///
/// [`Application`]: TODO
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Deserialize, Serialize)]
pub struct ApplicationId(pub Snowflake);

/// The ID of an [`Attachment`].
///
/// [`Attachment`]: TODO
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Deserialize, Serialize)]
pub struct AttachmentId(pub Snowflake);

/// The ID of an [`AuditLogEntry`].
///
/// [`AuditLogEntry`]: TODO
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Deserialize, Serialize)]
pub struct AuditLogEntryId(pub Snowflake);

/// The ID of a [`Channel`].
///
/// [`Channel`]: TODO
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Deserialize, Serialize)]
pub struct ChannelId(pub Snowflake);

/// The ID of an [`Emoji`].
///
/// [`Emoji`]: TODO
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Deserialize, Serialize)]
pub struct EmojiId(pub Snowflake);

/// The ID of a [`Guild`].
///
/// [`Guild`]: TODO
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Deserialize, Serialize)]
pub struct GuildId(pub Snowflake);

/// The ID of an [`Integration`].
///
/// [`Integration`]: TODO
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Deserialize, Serialize)]
pub struct IntegrationId(pub Snowflake);

/// The ID of a [`Message`].
///
/// [`Message`]: TODO
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Deserialize, Serialize)]
pub struct MessageId(pub Snowflake);

/// The ID of a [`Role`].
///
/// [`Role`]: TODO
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Deserialize, Serialize)]
pub struct RoleId(pub Snowflake);

/// The ID of a [`User`].
///
/// [`User`]: TODO
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Deserialize, Serialize)]
pub struct UserId(pub Snowflake);

/// The ID of a [`Webhook`].
///
/// [`Webhook`]: TODO
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Deserialize, Serialize)]
pub struct WebhookId(pub Snowflake);
