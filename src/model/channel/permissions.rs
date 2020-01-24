//! Channel permission overwrite models.

use std::fmt::{self, Display};

use serde::{Deserialize, Serialize};

use crate::model::id::{RoleId, ToSnowflakeId, UserId};
use crate::model::permissions::Permissions;
use crate::model::snowflake::{Snowflake, ToSnowflake};

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

#[doc(hidden)]
impl crate::model::id::private::Sealed for PermissionOverwrite {}

impl ToSnowflakeId for PermissionOverwrite {
    type Id = OverwriteId;

    fn id(&self) -> Self::Id {
        self.id
    }
}

#[doc(hidden)]
impl crate::model::snowflake::private::Sealed for OverwriteId {}

impl ToSnowflake for OverwriteId {
    fn snowflake(self) -> Snowflake {
        match self {
            OverwriteId::Role(id) => id.snowflake(),
            OverwriteId::User(id) => id.snowflake(),
        }
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn test_deserialize_role() {
        let value = json!({
            "id": "80351110224678912",
            "type": "role",
            "allow": 104188992,
            "deny": 135168,
        });
        let overwrites = PermissionOverwrite::new(
            RoleId::from(80351110224678912),
            Permissions::from_bits(104188992).expect("valid permissions"),
            Permissions::from_bits(135168).expect("valid permissions"),
        );

        assert_eq!(
            overwrites,
            PermissionOverwrite::deserialize(&value).unwrap()
        );
    }

    #[test]
    fn test_serialize_role() {
        let value = json!({
            "id": "80351110224678912",
            "type": "role",
            "allow": 104188992,
            "deny": 135168,
        });
        let overwrites = PermissionOverwrite::new(
            RoleId::from(80351110224678912),
            Permissions::from_bits(104188992).expect("valid permissions"),
            Permissions::from_bits(135168).expect("valid permissions"),
        );

        assert_eq!(value, serde_json::to_value(&overwrites).unwrap());
    }

    #[test]
    fn test_deserialize_user() {
        let value = json!({
            "id": "80351110224678912",
            "type": "member",
            "allow": 104188992,
            "deny": 135168,
        });
        let overwrites = PermissionOverwrite::new(
            UserId::from(80351110224678912),
            Permissions::from_bits(104188992).expect("valid permissions"),
            Permissions::from_bits(135168).expect("valid permissions"),
        );

        assert_eq!(
            overwrites,
            PermissionOverwrite::deserialize(&value).unwrap()
        );
    }

    #[test]
    fn test_serialize_user() {
        let value = json!({
            "id": "80351110224678912",
            "type": "member",
            "allow": 104188992,
            "deny": 135168,
        });
        let overwrites = PermissionOverwrite::new(
            UserId::from(80351110224678912),
            Permissions::from_bits(104188992).expect("valid permissions"),
            Permissions::from_bits(135168).expect("valid permissions"),
        );

        assert_eq!(value, serde_json::to_value(&overwrites).unwrap());
    }
}
