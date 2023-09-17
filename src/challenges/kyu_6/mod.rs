/*
Write a function that accepts an array of 10 integers (between 0 and 9), that returns a string of those numbers in the form of a phone number.
ex. create_phone_number(&[1,2,3,4,5,6,7,8,9,0]); // returns "(123) 456-7890"
The returned format must be correct in order to complete this challenge.
*/
pub fn concat_to_string(slice: &[u8]) -> String {
    slice.iter().map(|&x| x.to_string()).collect()
}

pub fn create_phone_number(numbers: &[u8]) -> String {
    let area: String = concat_to_string(&numbers[0..3]);
    let first_group: String = concat_to_string(&numbers[3..6]);
    let second_group: String = concat_to_string(&numbers[6..]);

    return format!("({}) {}-{}", area, first_group, second_group).to_string();
}

// ----------------------------------------------------------------
// FIND THE ODD INT KATA
// ----------------------------------------------------------------
/*
Given an array of integers, find the one that appears an odd number of times.
There will always be only one integer that appears an odd number of times.
*/

#[allow(dead_code)]
pub fn find_it(_seq: &[i32]) -> i32 {
    // let mut count = 0;
    // let mut odd = 0;
    // for i in 0..seq.len() {
    //     for j in 0..seq.len() {
    //         if seq[i] == seq[j] {
    //             count += 1;
    //         }
    //     }
    //     if count % 2 != 0 {
    //         odd = seq[i];
    //         break;
    //     }
    // }
    // return odd;
    todo!()
}
