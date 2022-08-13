use std::collections::HashSet;
use num::BigInt;

fn main() {
    let mut numbers = HashSet::<BigInt>::new();
    for a in 2..=100 {
        for b in 2..=100 {
            let c = BigInt::from(a).pow(b);
            numbers.insert(c);
        }
    }
    let ans = numbers.len();
    dbg!(ans);
}
