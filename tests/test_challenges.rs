use codewars_in_rust::{create_phone_number, valid_braces};

#[test]
fn test_create_phone_number() {
    assert_eq!(
        create_phone_number(&[1, 1, 1, 1, 1, 1, 1, 1, 1, 1]),
        "(111) 111-1111"
    );
    assert_eq!(
        create_phone_number(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 9]),
        "(123) 456-7899"
    );
}

#[test]
fn test_valid_braces() {
    assert_eq!(valid_braces(&"(){}[]".to_string()), true);
    assert_eq!(valid_braces(&"([{}])".to_string()), true);
    assert_eq!(valid_braces(&"(}".to_string()), false);
    assert_eq!(valid_braces(&"[(])".to_string()), false);
}
