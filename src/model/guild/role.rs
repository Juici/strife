use serde::{Deserialize, Serialize};

use crate::model::color::Color;
use crate::model::id::RoleId;
use crate::model::permissions::Permissions;

/// Represents a set of permissions attached to a group of users.
///
/// Roles have unique names, colors, and can be pinned to the side bar, causing
/// their members to be listed separately. Roles are unique per guild, and can
/// have separate permission profiles for the global context (guild) and channel
/// context.
///
/// The `@everyone` role has the same ID as the guild it belongs to.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Role {
    /// The ID of the role.
    pub id: RoleId,
    /// The name of the role.
    pub name: String,
    /// The color of the role.
    #[serde(default, alias = "colour")]
    pub color: Color,
    /// Whether the role is pinned in the user listing.
    #[serde(rename = "hoist")]
    pub pinned: bool,
    /// The position of the role.
    pub position: usize,
    /// The set of permissions for the users with the role.
    pub permissions: Permissions,
    /// Whether the role is managed by an integration.
    pub managed: bool,
    /// Whether the role is mentionable.
    pub mentionable: bool,
}

impl_eq_fields!(Role: [
    id,
    name,
    color,
    pinned,
    position,
    permissions,
    managed,
    mentionable,
]);

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn test_deserialize() {
        let value = json!({
          "id": "41771983423143936",
          "name": "WE DEM BOYZZ!!!!!!",
          "color": 3447003,
          "hoist": true,
          "position": 1,
          "permissions": 66321471,
          "managed": false,
          "mentionable": false
        });
        let role = Role {
            id: RoleId::from(41771983423143936),
            name: "WE DEM BOYZZ!!!!!!".to_owned(),
            color: Color::new(3447003),
            pinned: true,
            position: 1,
            permissions: Permissions::from_bits(66321471).unwrap(),
            managed: false,
            mentionable: false,
        };

        let deserialized = Role::deserialize(&value).unwrap();
        assert_eq_fields!(role, deserialized);
    }

    #[test]
    fn test_serialize() {
        let value = json!({
          "id": "41771983423143936",
          "name": "WE DEM BOYZZ!!!!!!",
          "color": 3447003,
          "hoist": true,
          "position": 1,
          "permissions": 66321471,
          "managed": false,
          "mentionable": false
        });
        let role = Role {
            id: RoleId::from(41771983423143936),
            name: "WE DEM BOYZZ!!!!!!".to_owned(),
            color: Color::new(3447003),
            pinned: true,
            position: 1,
            permissions: Permissions::from_bits(66321471).unwrap(),
            managed: false,
            mentionable: false,
        };

        assert_eq!(value, serde_json::to_value(role).unwrap());
    }
}
