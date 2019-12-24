use serde::{Deserialize, Serialize};

use crate::model::id::AttachmentId;

/// A file uploaded and attached to a [`Message`].
///
/// [`Message`]: struct.Message.html
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Attachment {
    /// The ID of the attachment.
    id: AttachmentId,
    /// The name of the file attached.
    filename: String,
    /// The size of the file in bytes.
    size: u64,
    /// The source URL of the file.
    url: String,
    /// The proxied URL of the file.
    proxy_url: String,
    /// Height of image, if the file is an image.
    height: Option<u64>,
    /// Width of image, if the file is an image.
    width: Option<u64>,
}
