use num::{BigInt, Integer, Zero};

fn main() {
    let mut n: BigInt = (1..=100).map(BigInt::from).product();
    let mut sum: BigInt = 0.into();
    let ten = 10.into();
    while !n.is_zero() {
        let (rem, div) = n.div_rem(&ten);
        sum += div;
        n = rem;
    }
    dbg!(sum);
}
