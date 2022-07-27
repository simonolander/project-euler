fn main() {
    let mut primes = Vec::new();
    let mut sieve = [true; 114320]; // n * log(n) + n * log(log(n))
    for p in 2..sieve.len() {
        if !sieve[p] {
            continue
        }
        primes.push(p);
        let mut m = 2*p;
        while m < sieve.len() {
            sieve[m] = false;
            m += p;
        }
    }
    dbg!(primes[10000]);
}
