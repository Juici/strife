//! Strongly-typed snowflake IDs.

use std::fmt::{self, Debug, Display};
use std::hash::Hash;
use std::ops::Deref;

use serde::{Deserialize, Serialize};

use crate::model::channel::message::Attachment;
use crate::model::channel::Message;
use crate::model::guild::{CustomEmoji, Emoji, Guild, PartialGuild, Role};
use crate::model::snowflake::{Snowflake, ToSnowflake};
use crate::model::user::{ClientUser, User};

macro_rules! id_type {
    ($(
        $(#[$attr:meta])*
        $Id:ident;
    )*) => {$(
        $(#[$attr])*
        #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Deserialize, Serialize)]
        pub struct $Id(Snowflake);

        impl Display for $Id {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                Display::fmt(&self.0, f)
            }
        }

        impl Deref for $Id {
            type Target = Snowflake;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl AsRef<Snowflake> for $Id {
            fn as_ref(&self) -> &Snowflake {
                &self.0
            }
        }

        impl From<u64> for $Id {
            fn from(n: u64) -> Self {
                Self(Snowflake::from(n))
            }
        }

        impl From<Snowflake> for $Id {
            fn from(snowflake: Snowflake) -> Self {
                Self(snowflake)
            }
        }

        impl From<$Id> for Snowflake {
            fn from(id: $Id) -> Self {
                id.0
            }
        }

        impl_to_snowflake!($Id: |id| id.0);
    )*};
}

id_type! {
    /// The ID of an [`Application`].
    ///
    /// [`Application`]: TODO
    ApplicationId;

    /// The ID of an [`Attachment`].
    ///
    /// [`Attachment`]: ../channel/struct.Attachment.html
    AttachmentId;

    /// The ID of an [`AuditLogEntry`].
    ///
    /// [`AuditLogEntry`]: TODO
    AuditLogEntryId;

    /// The ID of a [`Channel`].
    ///
    /// [`Channel`]: ../channel/enum.Channel.html
    ChannelId;

    /// The ID of an [`Emoji`].
    ///
    /// [`Emoji`]: ../guild/struct.Emoji.html
    EmojiId;

    /// The ID of a [`Guild`].
    ///
    /// [`Guild`]: ../guild/struct.Guild.html
    GuildId;

    /// The ID of an [`Integration`].
    ///
    /// [`Integration`]: TODO
    IntegrationId;

    /// The ID of a [`Message`].
    ///
    /// [`Message`]: ../channel/struct.Message.html
    MessageId;

    /// The ID of a [`Role`].
    ///
    /// [`Role`]: ../guild/struct.Role.html
    RoleId;

    /// The ID of a [`User`].
    ///
    /// [`User`]: ../user/struct.User.html
    UserId;

    /// The ID of a [`Webhook`].
    ///
    /// [`Webhook`]: TODO
    WebhookId;
}

pub(crate) mod private {
    pub trait Sealed {}
}

/// A trait that have a strongly-typed Snowflake ID.
pub trait ToSnowflakeId: private::Sealed {
    /// The strongly-typed Snowflake ID type.
    type Id: ToSnowflake + Copy + Debug + Display + Eq + Hash;

    /// Returns the Snowflake ID.
    fn id(&self) -> Self::Id;
}

macro_rules! impl_to_id {
    ($Parent:ident => $field:ident: $Id:ident) => {
        impl private::Sealed for $Parent {}

        impl ToSnowflakeId for $Parent {
            type Id = $Id;

            fn id(&self) -> Self::Id {
                self.$field
            }
        }

        impl_to_snowflake!($Parent: |parent| parent.id().snowflake());
    };
    ($(
        $Parent:ident => $field:ident: $Id:ident;
    )*) => {$(
        impl_to_id!($Parent => $field: $Id);
    )*};
}

impl_to_id! {
    Attachment => id: AttachmentId;
    Emoji => id: EmojiId;
    CustomEmoji => id: EmojiId;
    Guild => id: GuildId;
    PartialGuild => id: GuildId;
    Message => id: MessageId;
    Role => id: RoleId;
    User => id: UserId;
    ClientUser => id: UserId;
}

// TODO: Implement ToSnowflakeId for Channel.
// TODO: Implement ToSnowflakeId for other types.
