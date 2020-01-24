use strife::model::user::Discriminator;

#[test]
fn test_parse() {
    assert_eq!(
        Discriminator::new(0001).unwrap(),
        "0001".parse::<Discriminator>().unwrap()
    );
}

#[test]
fn test_new_unchecked() {
    assert_eq!(
        unsafe { Discriminator::new_unchecked(500) },
        Discriminator::new(500).unwrap()
    );
}

#[test]
fn test_invalid() {
    assert!(Discriminator::new(10000).is_err());
}
