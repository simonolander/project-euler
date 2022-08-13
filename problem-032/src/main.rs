use std::collections::HashSet;
use itertools::Itertools;

fn main() {
    let mut products: HashSet<u32> = HashSet::new();
    for perm in (1u32..=9).permutations(9) {
        for i in 1..perm.len() {
            let a = number(&perm[0..i]);
            for j in i..perm.len() {
                let b = number(&perm[i..j]);
                let c = number(&perm[j..]);
                if a * b == c {
                    products.insert(c);
                }
            }
        }
    }
    let ans: u32 = products.iter().sum();
    dbg!(ans);
}

fn number(digits: &[u32]) -> u32 {
    let mut n: u32 = 0;
    for d in digits {
        n *= 10;
        n += d;
    }
    n
}
