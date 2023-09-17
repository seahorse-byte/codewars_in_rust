use crate::challenges::kyu_6::create_phone_number;
mod challenges;

fn main() {
    // CREATE PHONE NUMBER KATA
    let answer = create_phone_number(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 9]);
    assert_eq!(create_phone_number(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 9]), answer);

    // FIND THE ODD INT KATA
    // let answer = find_it(&[1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 5]);
    // assert_eq!(find_it(&[1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 5]), answer);
    
}
