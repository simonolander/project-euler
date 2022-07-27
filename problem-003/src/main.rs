fn main() {
    let factors = factorize(600851475143);
    let largest_factor = factors.last().unwrap();
    dbg!(largest_factor);
}

fn factorize(n: i64) -> Vec<i64> {
    let mut remainder = n;
    let mut factors = Vec::new();
    for p in 2..n {
        while remainder % p == 0 {
            factors.push(p);
            remainder /= p;
        }
        if remainder == 1 {
            break
        }
    }
    factors
}
