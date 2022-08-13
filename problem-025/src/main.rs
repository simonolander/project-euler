use std::mem::swap;

use num::{BigInt, One};

fn main() {
    let mut f1: BigInt = BigInt::one();
    let mut f2: BigInt = BigInt::one();
    let mut index: usize = 2;
    while f2.to_string().len() < 1000 {
        f1 += &f2;
        swap(&mut f1, &mut f2);
        index += 1;
    }
    dbg!(index);
}
