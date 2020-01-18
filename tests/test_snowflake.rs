use serde::Deserialize;
use serde_json::json;
use strife::model::snowflake::Snowflake;

#[test]
fn test_serialize() {
    let value = json!("80351110224678912");
    let snowflake = Snowflake::from(80351110224678912);
    assert_eq!(value, serde_json::to_value(&snowflake).unwrap());
}

#[test]
fn test_deserialize() {
    let snowflake = Snowflake::from(80351110224678912);

    let value = json!(80351110224678912u64);
    assert_eq!(snowflake, Snowflake::deserialize(&value).unwrap());

    let value = json!("80351110224678912");
    assert_eq!(snowflake, Snowflake::deserialize(&value).unwrap());
}
