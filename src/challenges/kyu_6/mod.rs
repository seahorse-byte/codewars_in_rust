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
// VALID BRACES
// ----------------------------------------------------------------
/*
Write a function that takes a string of braces, and determines if the order of the braces is valid. It should return true if the string is valid, and false if it's invalid.

This Kata is similar to the Valid Parentheses Kata, but introduces new characters: brackets [], and curly braces {}. Thanks to @arnedag for the idea!

All input strings will be nonempty, and will only consist of parentheses, brackets and curly braces: ()[]{}.

What is considered Valid?
A string of braces is considered valid if all braces are matched with the correct brace.
Examples
"(){}[]"   =>  True
"([{}])"   =>  True
"(}"       =>  False
"[(])"     =>  False
"[({})](]" =>  False
*/

pub fn valid_braces(mut s: String) -> bool {
    for _ in 0..s.len() {
        s = s.replace("()", "").replace("[]", "").replace("{}", "");
    }
    s.len() == 0
}
