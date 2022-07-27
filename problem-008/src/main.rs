use std::iter::zip;

fn main() {
    let n = 13;
    let digits: Vec<_> = include_str!("input.txt")
        .chars()
        .filter(|c| c.is_ascii_digit())
        .map(|c| c.to_string().parse::<i64>().unwrap())
        .collect();
    let product = zip(0..digits.len() - n, n..digits.len())
        .map(|(from, to)| &digits[from..to])
        .map(|ds| ds.iter().product::<i64>())
        .max();
    dbg!(product);
}
