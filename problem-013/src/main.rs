use num::BigUint;

fn main() {
    let numbers: Vec<BigUint> = include_str!("input.txt")
        .lines()
        .map(|str| str.parse().unwrap())
        .collect();
    let sum: BigUint = numbers.iter().sum();
    let first_ten: String = sum.to_string().chars().take(10).collect();
    dbg!(sum, first_ten);
}
