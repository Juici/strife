//! Strongly-typed snowflake IDs.

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
        impl<T> From<T> for $name
        where
            T: Into<Snowflake>,
        {
            fn from(into: T) -> Self {
                Self(into.into())
            }
        }

        impl Deref for $name {
            type Target = Snowflake;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                std::fmt::Display::fmt(&self.0, f)
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
