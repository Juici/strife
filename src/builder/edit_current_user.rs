use serde::Serialize;

use crate::model::image::ImageDataRef;

/// A builder for editing the client user.
#[derive(Debug, Serialize)]
pub struct EditCurrentUser<'a> {
    #[serde(rename = "username")]
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    avatar: Option<&'a ImageDataRef>,
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
    pub fn avatar(&mut self, avatar: &'a ImageDataRef) {
        self.avatar = Some(avatar);
    }
}
