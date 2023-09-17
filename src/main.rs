use crate::challenges::kyu_6::create_phone_number;
mod challenges;

fn main() {
    let answer = create_phone_number(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 9]);
    // assert_eq!(create_phone_number(&[1, 1, 1, 1, 1, 1, 1, 1, 1, 1]), "(111) 111-1111");
    // assert_eq!(create_phone_number(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 9]), "(123) 456-7899");
    println!("answer is {}", answer);
}
