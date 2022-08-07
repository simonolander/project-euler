use num::{Integer, Zero};
use num_bigint::BigUint;

fn main() {
    let mut n = BigUint::from(2u8).pow(1000);
    let mut sum = BigUint::zero();
    let ten = BigUint::from(10u8);
    while !n.is_zero() {
        let (remainder, modulus) = n.div_mod_floor(&ten);
        n = remainder;
        sum += modulus;
    }
    dbg!(sum);
}
