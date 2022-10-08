use eutils::int::{is_palindrome_10, is_palindrome_2};

fn main() {
    let ans: u64 = (0..1000000).into_iter()
        .filter(is_palindrome_2)
        .filter(is_palindrome_10)
        .sum();

    dbg!(ans);
}
