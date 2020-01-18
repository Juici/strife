use serde::Deserialize;
use serde_json::json;
use strife::model::user::Discriminator;

#[test]
fn test_parse1() {
    let d = Discriminator::new(0001).unwrap();

    assert_eq!(d, "0001".parse::<Discriminator>().unwrap());
    assert_eq!(d, "001".parse::<Discriminator>().unwrap());
    assert_eq!(d, "01".parse::<Discriminator>().unwrap());
    assert_eq!(d, "1".parse::<Discriminator>().unwrap());
}

#[test]
fn test_parse2() {
    let d = Discriminator::new(369).unwrap();

    assert_eq!(d, "0369".parse::<Discriminator>().unwrap());
    assert_eq!(d, "369".parse::<Discriminator>().unwrap());
}

#[test]
fn test_new_unchecked() {
    let d1 = unsafe { Discriminator::new_unchecked(500) };

    let d2 = Discriminator::new(500).unwrap();
    let d3 = "0500".parse::<Discriminator>().unwrap();

    assert_eq!(d1, d2);
    assert_eq!(d1, d3);
}

#[test]
fn test_invalid() {
    assert!(Discriminator::new(10000).is_err());
}

#[test]
fn test_serialize() {
    let value = json!("0001");
    let d = Discriminator::new(1).unwrap();

    assert_eq!(value, serde_json::to_value(&d).unwrap());
}

#[test]
fn test_deserialize() {
    let value = json!("0001");
    let d = Discriminator::new(1).unwrap();

    assert_eq!(d, Discriminator::deserialize(&value).unwrap());
}
