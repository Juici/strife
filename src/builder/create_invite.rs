use serde::Serialize;

/// A builder for creating a new invite.
#[derive(Debug, Serialize)]
pub struct CreateInvite {
    #[serde(skip_serializing_if = "Option::is_none")]
    max_age: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_uses: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    temporary: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    unique: Option<bool>,
}

impl CreateInvite {
    pub(crate) fn create() -> Self {
        CreateInvite {
            max_age: None,
            max_uses: None,
            temporary: None,
            unique: None,
        }
    }

    /// Sets the duration of the invite in seconds before expiry, or `0` to
    /// never expire.
    pub fn max_age(&mut self, max_age: u64) {
        self.max_age = Some(max_age);
    }

    /// Sets the maximum number of uses, or `0` for unlimited.
    pub fn max_uses(&mut self, max_uses: u64) {
        self.max_uses = Some(max_uses);
    }

    /// Sets whether the invite only grants temporary membership.
    pub fn temporary(&mut self, temporary: bool) {
        self.temporary = Some(temporary);
    }

    /// Sets whether to create a new invite, or to try and reuse a similar
    /// invite.
    pub fn unique(&mut self, unique: bool) {
        self.unique = Some(unique);
    }
}
