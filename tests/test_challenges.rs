use codewars_in_rust::{create_phone_number, is_valid_walk, valid_braces};

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

#[test]
fn test_is_valid_walk() {
    let walk = is_valid_walk(&['n', 's', 'n', 's', 'n', 's', 'n', 's', 'n', 's']);
    assert_eq!(walk, true);
    let walk2 = is_valid_walk(&['w', 'e', 'w', 'e', 'w', 'e', 'w', 'e', 'w', 'e', 'w', 'e']);
    assert_eq!(walk2, false);
    let walk3 = is_valid_walk(&['w']);
    assert_eq!(walk3, false);
    let walk4 = is_valid_walk(&['n', 'n', 'n', 's', 'n', 's', 'n', 's', 'n', 's']);
    assert_eq!(walk4, false);
}
