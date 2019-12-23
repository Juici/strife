//! Strongly-typed snowflake IDs.

use std::fmt::{self, Display};
use std::ops::Deref;

use serde::{Deserialize, Serialize};

use crate::model::snowflake::Snowflake;

/// The ID of an [`Application`].
///
/// [`Application`]: TODO
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Deserialize, Serialize)]
pub struct ApplicationId(Snowflake);

/// The ID of an [`Attachment`].
///
/// [`Attachment`]: TODO
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Deserialize, Serialize)]
pub struct AttachmentId(Snowflake);

/// The ID of an [`AuditLogEntry`].
///
/// [`AuditLogEntry`]: TODO
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Deserialize, Serialize)]
pub struct AuditLogEntryId(Snowflake);

/// The ID of a [`Channel`].
///
/// [`Channel`]: TODO
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Deserialize, Serialize)]
pub struct ChannelId(Snowflake);

/// The ID of an [`Emoji`].
///
/// [`Emoji`]: TODO
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Deserialize, Serialize)]
pub struct EmojiId(Snowflake);

/// The ID of a [`Guild`].
///
/// [`Guild`]: TODO
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Deserialize, Serialize)]
pub struct GuildId(Snowflake);

/// The ID of an [`Integration`].
///
/// [`Integration`]: TODO
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Deserialize, Serialize)]
pub struct IntegrationId(Snowflake);

/// The ID of a [`Message`].
///
/// [`Message`]: TODO
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Deserialize, Serialize)]
pub struct MessageId(Snowflake);

/// The ID of a [`Role`].
///
/// [`Role`]: TODO
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Deserialize, Serialize)]
pub struct RoleId(Snowflake);

/// The ID of a [`User`].
///
/// [`User`]: TODO
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Deserialize, Serialize)]
pub struct UserId(Snowflake);

/// The ID of a [`Webhook`].
///
/// [`Webhook`]: TODO
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Deserialize, Serialize)]
pub struct WebhookId(Snowflake);

macro_rules! impl_id {
    ($($name:ident,)*) => {$(
        impl From<u64> for $name {
            fn from(n: u64) -> Self {
                Self(Snowflake::from(n))
            }
        }

        impl From<$name> for Snowflake {
            fn from(id: $name) -> Self {
                id.0
            }
        }

        impl Deref for $name {
            type Target = Snowflake;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl AsRef<Snowflake> for $name {
            fn as_ref(&self) -> &Snowflake {
                &self.0
            }
        }

        impl Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                Display::fmt(&self.0, f)
            }
        }
    )*}
}

impl_id! {
    ApplicationId,
    AttachmentId,
    AuditLogEntryId,
    ChannelId,
    EmojiId,
    GuildId,
    IntegrationId,
    MessageId,
    RoleId,
    UserId,
    WebhookId,
}
