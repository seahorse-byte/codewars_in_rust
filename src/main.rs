use crate::challenges::kyu_6::create_phone_number;
mod challenges;

fn main() {
    let answer = create_phone_number(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 9]);
    println!("answer is {}", answer);
}
