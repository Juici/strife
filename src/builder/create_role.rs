use serde::Serialize;

use crate::model::color::Color;
use crate::model::permissions::Permissions;

/// A builder for creating a new role.
#[derive(Debug, Serialize)]
pub struct CreateRole<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    permissions: Option<Permissions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    color: Option<Color>,
    #[serde(rename = "hoist")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pinned: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mentionable: Option<bool>,
}

impl<'a> CreateRole<'a> {
    pub(crate) fn new() -> Self {
        CreateRole {
            name: None,
            permissions: None,
            color: None,
            pinned: None,
            mentionable: None,
        }
    }

    /// Sets the name of the role.
    pub fn name(&mut self, name: &'a str) {
        self.name = Some(name);
    }

    /// Sets the permissions of the role.
    pub fn permissions(&mut self, permissions: Permissions) {
        self.permissions = Some(permissions);
    }

    /// Sets the color of the role.
    pub fn color(&mut self, color: Color) {
        self.color = Some(color);
    }

    /// Sets whether the role should be pinned in the user list.
    pub fn pinned(&mut self, pinned: bool) {
        self.pinned = Some(pinned);
    }

    /// Sets whether the role can be mentioned.
    pub fn mentionable(&mut self, mentionable: bool) {
        self.mentionable = Some(mentionable);
    }
}
