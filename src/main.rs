use crate::challenges::kyu_6::{create_phone_number, is_valid_walk, valid_braces};
mod challenges;

fn main() {
    // CREATE PHONE NUMBER KATA
    let phone = create_phone_number(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 9]);
    assert_eq!(create_phone_number(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 9]), phone);

    // VALID BRACES KATA
    let braces = valid_braces("(){}[]");
    assert_eq!(braces, true);

    // IS VALID WALK
    let walk = is_valid_walk(&['n', 's', 'n', 's', 'n', 's', 'n', 's', 'n', 's']);
    assert_eq!(walk, true);
}
