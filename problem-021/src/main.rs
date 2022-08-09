fn main() {
    let mut sieve = vec![0; 100_000];
    for n in 1..sieve.len() {
        let mut m = 2 * n;
        while m < sieve.len() {
            sieve[m] += n;
            m += n;
        }
    }

    let mut ans = 0;
    for n in 0..10000 {
        let m = sieve[n];
        if n == m {
            continue
        }
        if n == sieve[m] {
            ans += n;
        }
    }

    dbg!(ans);
}
