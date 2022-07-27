fn main() {
    dbg!(lcm(1..=20));
}

fn lcm<T>(ns: T) -> i32 where T: Iterator<Item=i32> {
    let mut m = 1;
    for n in ns {
        m *= n / gcd(m, n);
    }
    m
}

fn gcd(a: i32, b: i32) -> i32 {
    if a > b {
        gcd(b, a)
    } else if a == 0 {
        b
    } else {
        gcd(b % a, a)
    }
}
