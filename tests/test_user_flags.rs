use strife::model::user::UserFlags;

#[test]
fn test_default_empty() {
    assert_eq!(UserFlags::default(), UserFlags::empty());
}
