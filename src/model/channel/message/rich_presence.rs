use int_enum::IntEnum;
use serde::{Deserialize, Serialize};

use crate::model::id::ApplicationId;

/// Rich Presence activity information.
#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct MessageActivity {
    /// Type of message activity.
    #[serde(rename = "type")]
    pub kind: MessageActivityType,
    /// The `party_id` from a Rich Presence event.
    pub party_id: Option<String>,
}

/// Type of a [`MessageActivity`].
///
/// [`Message`]: struct.MessageActivity.html
#[allow(missing_docs)]
#[repr(u8)]
#[non_exhaustive]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, IntEnum)]
pub enum MessageActivityType {
    Join = 1,
    Spectate = 2,
    Listen = 3,
    JoinRequest = 5,
}

/// Rich Presence application information.
#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct MessageApplication {
    /// The ID of the application.
    pub id: ApplicationId,
    /// The ID of the embed image asset.
    pub cover_image: Option<String>,
    /// The description of the application.
    pub description: String,
    /// The ID of the application icon.
    pub icon: Option<String>,
    /// The name of the application.
    pub name: String,
}
