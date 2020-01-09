use std::fmt::{self, Display};

use serde::{Deserialize, Serialize};

use crate::model::id::{RoleId, UserId};
use crate::model::permissions::Permissions;

/// The ID of a [`PermissionOverwrite`].
///
/// [`PermissionOverwrite`]: struct.PermissionOverwrite.html
#[non_exhaustive]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Deserialize, Serialize)]
#[serde(tag = "type", content = "id")]
pub enum OverwriteId {
    /// A role with permission overwrites being edited.
    #[serde(rename = "role")]
    Role(RoleId),
    /// A user with permission overwrites being edited.
    #[serde(rename = "member")]
    User(UserId),
}

impl Display for OverwriteId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            OverwriteId::Role(id) => id.fmt(f),
            OverwriteId::User(id) => id.fmt(f),
        }
    }
}

impl From<RoleId> for OverwriteId {
    fn from(id: RoleId) -> Self {
        OverwriteId::Role(id)
    }
}

impl From<UserId> for OverwriteId {
    fn from(id: UserId) -> Self {
        OverwriteId::User(id)
    }
}

/// Channel-specific permission overwrites for a role or user.
#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct PermissionOverwrite {
    /// The ID of the role or user.
    #[serde(flatten)]
    pub id: OverwriteId,
    /// The set of permissions being allowed.
    pub allow: Permissions,
    /// The set of permissions being denied.
    pub deny: Permissions,
}

impl PermissionOverwrite {
    /// Create a `PermissionOverwrite` with empty permission overwrites.
    pub fn empty<Id>(id: Id) -> PermissionOverwrite
    where
        Id: Into<OverwriteId>,
    {
        let id = id.into();
        PermissionOverwrite {
            id,
            allow: Permissions::empty(),
            deny: Permissions::empty(),
        }
    }

    /// Create a `PermissionOverwrite` with given permission overwrites.
    pub fn new<Id>(id: Id, allow: Permissions, deny: Permissions) -> PermissionOverwrite
    where
        Id: Into<OverwriteId>,
    {
        let id = id.into();
        PermissionOverwrite { id, allow, deny }
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    const ID: u64 = 80351110224678912;

    const ALLOW_BITS: u64 = 104188992;
    const DENY_BITS: u64 = 135168;

    #[test]
    fn test_serialize_role() {
        let allow = Permissions::from_bits(ALLOW_BITS).expect("valid permissions");
        let deny = Permissions::from_bits(DENY_BITS).expect("valid permissions");

        let overwrites = PermissionOverwrite::new(RoleId::from(ID), allow, deny);

        let expected = json!({
            "id": "80351110224678912",
            "type": "role",
            "allow": 104188992,
            "deny": 135168,
        });

        let v = serde_json::to_value(overwrites).unwrap();
        assert_eq!(v, expected);
    }

    #[test]
    fn test_serialize_user() {
        let allow = Permissions::from_bits(ALLOW_BITS).expect("valid permissions");
        let deny = Permissions::from_bits(DENY_BITS).expect("valid permissions");

        let overwrites = PermissionOverwrite::new(UserId::from(ID), allow, deny);

        let expected = json!({
            "id": "80351110224678912",
            "type": "member",
            "allow": 104188992,
            "deny": 135168,
        });

        let v = serde_json::to_value(overwrites).unwrap();
        assert_eq!(v, expected);
    }
}
