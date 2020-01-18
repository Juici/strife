use serde::Deserialize;
use serde_json::json;
use strife::model::channel::permissions::PermissionOverwrite;
use strife::model::id::{RoleId, UserId};
use strife::model::permissions::Permissions;

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
