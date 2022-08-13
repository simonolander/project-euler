use std::collections::HashSet;

fn main() {
    let mut sieve = vec![0; 30_000];
    let mut abundant_numbers = Vec::new();
    for n in 1..sieve.len() {
        let mut m = 2 * n;
        while m < sieve.len() {
            sieve[m] += n;
            m += n;
        }
        if n < sieve[n] {
            abundant_numbers.push(n);
        }
    }
    let mut non_abundant_sums: HashSet<usize> = HashSet::from_iter(0..30_000);
    for i in 0..abundant_numbers.len() {
        for j in i..abundant_numbers.len() {
            if i + j > 30_000 {
                break;
            }
            let sum = abundant_numbers[i] + abundant_numbers[j];
            non_abundant_sums.remove(&sum);
        }
    }

    let ans: usize = non_abundant_sums.iter().sum();

    dbg!(ans);
}
