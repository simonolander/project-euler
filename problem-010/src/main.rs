fn primes(n: usize) -> Vec<usize> {
    let f = n as f64;
    let ceiling = (f * f.ln() + f * f.ln().ln()).ceil() as usize;
    let mut sieve = vec![true; ceiling];
    let mut primes = Vec::new();
    for p in 2..sieve.len() {
        if sieve[p] {
            primes.push(p);
            let mut m = 2*p;
            while m < sieve.len() {
                sieve[m] = false;
                m += p;
            }
        }
    }
    primes
}

fn main() {
    let sum = primes(2000000).iter().take_while(|&&p| p < 2000000).sum::<usize>();
    dbg!(sum);
}
