fn main() {
    let mut sieve = vec![false; 5_000_000];
    let mut primes = Vec::new();
    for p in 2..sieve.len() {
        if sieve[p] {
            continue;
        }
        primes.push(p);
        let mut m = p;
        while m < sieve.len() {
            sieve[m] = true;
            m += p;
        }
    }
    let mut triangle_number = 0;
    for n in 1usize.. {
        triangle_number += n;
        let mut number_of_divisors = 1;
        let mut c = triangle_number;
        for &p in primes.iter().take_while(|&&p| p < triangle_number) {
            if c == 1 {
                break
            }
            let mut d = 1;
            while c % p == 0 {
                c /= p;
                d += 1;
            }
            number_of_divisors *= d;
        }
        if number_of_divisors > 500 {
            dbg!(triangle_number, number_of_divisors);
            break;
        }
    }
}
