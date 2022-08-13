extern crate core;

use std::collections::HashSet;

fn main() {
    let limit = 1_000_000;
    let primes: HashSet<_> = HashSet::from_iter(primes_smaller_than(limit as usize));
    let mut max_n = 0;
    let mut ans = 0;
    for a in -999..=999 {
        for b in -1000..=1000 {
            for n in 0.. {
                let p = n * n + a * n + b;
                if p > limit {
                    panic!("prime too big: {}", p);
                }
                if !primes.contains(&p) {
                    if n > max_n {
                        max_n = n;
                        ans = a * b;
                    }
                    break;
                }
            }
        }
    }

    dbg!(ans);
}

fn primes_smaller_than(n: usize) -> Vec<i128> {
    let mut primes = Vec::new();
    let mut sieve = vec![true; n];
    for p in 2..sieve.len() {
        if sieve[p] {
            primes.push(p as i128);
        }
        let mut m = 2 * p;
        while m < sieve.len() {
            sieve[m] = false;
            m += p;
        }
    }
    primes
}
