use itertools::Itertools;

fn main() {
    let largest_palindrome = (100..1000)
        .cartesian_product(100..1000)
        .map(|(a, b)| a * b)
        .filter(is_palindrome)
        .max();
    dbg!(largest_palindrome);
}

fn is_palindrome(n: &i32) -> bool {
    let string = n.to_string();
    string.chars().rev().eq(string.chars())
}
