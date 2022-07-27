struct Fib {
    a: i32,
    b: i32,
}

impl Iterator for Fib {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        let (a, b) = (self.a, self.b);
        self.a = b;
        self.b = a + b;
        a.into()
    }
}

impl Fib {
    fn new(a: i32, b: i32) -> Fib {
        Fib { a, b }
    }
}

fn main() {
    let sum = Fib::new(1, 2)
        .take_while(|n| *n <= 4_000_000)
        .filter(|n| n % 2 == 0)
        .sum::<i32>();
    dbg!(sum);
}