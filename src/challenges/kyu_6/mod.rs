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

// pub fn valid_braces(mut s: String) -> bool {
//     for _ in 0..s.len() {
//         s = s.replace("()", "").replace("[]", "").replace("{}", "");
//     }
//     s.len() == 0
// }

pub fn valid_braces(s: &str) -> bool {
    let mut s = s.to_string();
    for _ in 0..s.len() {
        s = s.replace("()", "").replace("[]", "").replace("{}", "");
    }
    s.len() == 0
}

// ----------------------------------------------------------------
// IS VALID WALK
// ----------------------------------------------------------------
/* You live in the city of Cartesia where all roads are laid out in a perfect grid. You arrived ten minutes too early to an appointment, so you decided to take the opportunity to go for a short walk. The city provides its citizens with a Walk Generating App on their phones -- everytime you press the button it sends you an array of one-letter strings representing directions to walk (eg. ['n', 's', 'w', 'e']). You always walk only a single block for each letter (direction) and you know it takes you one minute to traverse one city block, so create a function that will return true if the walk the app gives you will take you exactly ten minutes (you don't want to be early or late!) and will, of course, return you to your starting point. Return false otherwise.

Note: you will always receive a valid array containing a random assortment of direction letters ('n', 's', 'e', or 'w' only). It will never give you an empty array (that's not a walk, that's standing still!).
*/

pub fn is_valid_walk(array: &[&str]) -> bool {
    if array.len() != 10 {
        return false;
    }

    let mut x = 0;
    let mut y = 0;

    for &direction in array {
        match direction {
            "n" => y += 1,
            "s" => y -= 1,
            "e" => x += 1,
            "w" => x -= 1,
            _ => (),
        }
    }

    x == 0 && y == 0
}
