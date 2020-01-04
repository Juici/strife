use serde::{Deserialize, Serialize};

use crate::model::id::AttachmentId;

/// A file uploaded and attached to a [`Message`].
///
/// [`Message`]: struct.Message.html
#[non_exhaustive]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Attachment {
    /// The ID of the attachment.
    pub id: AttachmentId,
    /// The name of the file attached.
    pub filename: String,
    /// The size of the file in bytes.
    pub size: u64,
    /// The source URL of the file.
    pub url: String,
    /// The proxied URL of the file.
    pub proxy_url: String,
    /// The height of the image, if the file is an image.
    pub height: Option<u64>,
    /// The width of the image, if the file is an image.
    pub width: Option<u64>,
}
