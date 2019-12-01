use serde::{Serialize, Serializer};

use crate::model::{
    id::{RoleId, UserId},
    permissions::Permissions,
};

/// The ID of a [`PermissionOverwrite`].
///
/// [`PermissionOverwrite`]: struct.PermissionOverwrite.html
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum OverwriteId {
    /// A role with permission overwrites being edited.
    Role(RoleId),
    /// A user with permission overwrites being edited.
    User(UserId),
    #[doc(hidden)]
    __Nonexhaustive,
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
#[derive(Clone, Debug, Serialize)]
pub struct PermissionOverwrite {
    /// The ID of the role or user.
    #[serde(rename = "type", serialize_with = "serialize_type")]
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

fn serialize_type<S>(id: &OverwriteId, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let r#type = match id {
        OverwriteId::Role(_) => "role",
        OverwriteId::User(_) => "member",
        OverwriteId::__Nonexhaustive => unreachable!(),
    };
    serializer.serialize_str(r#type)
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
        let id = OverwriteId::from(RoleId::from(ID));
        let allow = Permissions::from_bits(ALLOW_BITS).expect("valid permissions");
        let deny = Permissions::from_bits(DENY_BITS).expect("valid permissions");
        let overwrites = PermissionOverwrite::new(id, allow, deny);

        let expected = json!({
            "type": "role",
            "allow": 104188992,
            "deny": 135168,
        });

        let v = serde_json::to_value(overwrites).unwrap();
        assert_eq!(v, expected);
    }

    #[test]
    fn test_serialize_user() {
        let id = OverwriteId::from(UserId::from(ID));
        let allow = Permissions::from_bits(ALLOW_BITS).expect("valid permissions");
        let deny = Permissions::from_bits(DENY_BITS).expect("valid permissions");
        let overwrites = PermissionOverwrite::new(id, allow, deny);

        let expected = json!({
            "type": "member",
            "allow": 104188992,
            "deny": 135168,
        });

        let v = serde_json::to_value(overwrites).unwrap();
        assert_eq!(v, expected);
    }
}
