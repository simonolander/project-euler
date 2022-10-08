use num_prime::nt_funcs::{is_prime64, primes};

fn main() {
    let ans = primes(1000000).iter()
        .copied()
        .filter(is_circular)
        .count();
    dbg!(ans);
}

fn is_circular(n: &u64) -> bool {
    let mut n = *n;
    let mut digits: Vec<u64> = vec![];
    while n != 0 {
        digits.push(n % 10);
        n /= 10;
    }
    digits.reverse();
    for start in 0..digits.len() {
        let mut r = 0;
        for offset in 0..digits.len() {
            let index = (start + offset) % digits.len();
            r *= 10;
            r += digits[index];
        }
        if !is_prime64(r) {
            return false;
        }
    }
    return true;
}
