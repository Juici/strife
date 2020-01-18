use serde::Deserialize;
use serde_json::json;
use strife::model::guild::PartialEmoji;

#[test]
fn test_deserialize_standard() {
    let value = json!({
        "id": null,
        "name": "🔥",
    });
    let emoji = PartialEmoji::standard("🔥");

    assert_eq!(emoji, PartialEmoji::deserialize(&value).unwrap());
}

#[test]
fn test_serialize_standard() {
    let value = json!({
        "id": null,
        "name": "🔥",
    });
    let emoji = PartialEmoji::standard("🔥");

    assert_eq!(value, serde_json::to_value(&emoji).unwrap());
}

#[test]
fn test_deserialize_custom() {
    let value = json!({
        "id": "41771983429993937",
        "name": "LUL",
    });
    let emoji = PartialEmoji::custom(41771983429993937, "LUL", false);

    assert_eq!(emoji, PartialEmoji::deserialize(&value).unwrap());
}

#[test]
fn test_serialize_custom() {
    let value = json!({
        "id": "41771983429993937",
        "name": "LUL",
        "animated": true,
    });
    let emoji = PartialEmoji::custom(41771983429993937, "LUL", true);

    assert_eq!(value, serde_json::to_value(&emoji).unwrap());
}
