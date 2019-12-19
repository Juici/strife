//! Models related to channels.

mod message;
mod permission_overwrite;

pub use self::message::Message;
pub use self::permission_overwrite::{OverwriteId, PermissionOverwrite};
