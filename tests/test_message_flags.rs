use serde::Deserialize;
use serde_json::json;
use strife::model::channel::message::MessageFlags;

#[test]
fn test_deserialize_message_flags() {
    let value = json!(2);
    let flags = MessageFlags::IS_CROSSPOST;

    assert_eq!(flags, MessageFlags::deserialize(&value).unwrap());
}

#[test]
fn test_serialize_message_flags() {
    let value = json!(2);
    let flags = MessageFlags::IS_CROSSPOST;

    assert_eq!(value, serde_json::to_value(&flags).unwrap());
}
