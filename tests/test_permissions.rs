use serde::Deserialize;
use serde_json::json;
use strife::model::permissions::Permissions;

#[test]
fn test_all() {
    assert_eq!(
        Permissions::all(),
        Permissions::from_bits(2146959359).unwrap()
    );
}

#[test]
fn test_default() {
    assert_eq!(Permissions::default(), Permissions::empty());
}

#[test]
fn test_serialize() {
    let value = json!(103877696);
    let perms = Permissions::from_bits(103877696).unwrap();

    assert_eq!(value, serde_json::to_value(&perms).unwrap());
}

#[test]
fn test_deserialize() {
    let value = json!(68608);
    let perms =
        Permissions::VIEW_CHANNEL | Permissions::READ_MESSAGE_HISTORY | Permissions::SEND_MESSAGES;

    assert_eq!(perms, Permissions::deserialize(&value).unwrap());
}

#[test]
fn test_deserialize_invalid() {
    let value = json!(0x00080000);
    let err = Permissions::deserialize(&value);

    assert!(err.is_err());
    assert!(err.unwrap_err().is_data());
}
