use serde::Serialize;

use crate::model::image::ImageData;

/// A builder for editing the client user.
#[derive(Debug, Serialize)]
pub struct EditCurrentUser<'a> {
    #[serde(rename = "username")]
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    avatar: Option<ImageDataWrapper<'a>>,
}

impl<'a> EditCurrentUser<'a> {
    pub(crate) fn create() -> Self {
        EditCurrentUser {
            name: None,
            avatar: None,
        }
    }

    /// Sets the username of the client user.
    pub fn name(&mut self, name: &'a str) {
        self.name = Some(name);
    }

    /// Sets the avatar of the client user.
    pub fn avatar(&mut self, avatar: ImageData) {
        self.avatar = Some(ImageDataWrapper::ImageData(avatar));
    }

    /// Sets the avatar of the client user from a [Data URI scheme].
    ///
    /// Supports GIF, JPEG and PNG formats.
    ///
    /// An example Data URI format:
    /// ```text
    /// data:image/jpeg;base64,BASE64_ENCODED_JPEG_IMAGE_DATA
    /// ```
    ///
    /// # Safety
    ///
    /// No checks are made that Data URI scheme is valid and supported, as a
    /// result it is possible for the Discord API to to return errors due to
    /// invalid an image format.
    pub unsafe fn avatar_unchecked(&mut self, avatar: &'a str) {
        self.avatar = Some(ImageDataWrapper::Raw(avatar));
    }
}

#[derive(Debug, Serialize)]
#[serde(untagged)]
enum ImageDataWrapper<'a> {
    Raw(&'a str),
    ImageData(ImageData),
}
