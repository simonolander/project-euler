use memoize::memoize;

#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
enum Coin {
    P200,
    P100,
    P50,
    P20,
    P10,
    P5,
    P2,
    P1,
}

impl Coin {
    fn value(&self) -> u32 {
        match self {
            Coin::P200 => 200,
            Coin::P100 => 100,
            Coin::P50 => 50,
            Coin::P20 => 20,
            Coin::P10 => 10,
            Coin::P5 => 5,
            Coin::P2 => 2,
            Coin::P1 => 1,
        }
    }

    fn next(&self) -> Option<Coin> {
        match self {
            Coin::P200 => Some(Coin::P100),
            Coin::P100 => Some(Coin::P50),
            Coin::P50 => Some(Coin::P20),
            Coin::P20 => Some(Coin::P10),
            Coin::P10 => Some(Coin::P5),
            Coin::P5 => Some(Coin::P2),
            Coin::P2 => Some(Coin::P1),
            Coin::P1 => None,
        }
    }
}

fn main() {
    let ans = count(200, Coin::P200);
    dbg!(ans);
}

#[memoize]
fn count(n: u32, c: Coin) -> u32 {
    match c.next() {
        Some(next) => {
            let mut sum = count(n, next);
            if n >= c.value() {
                sum += count(n - c.value(), c);
            }
            sum
        }
        None => 1
    }
}
