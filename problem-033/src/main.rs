use std::collections::HashSet;
use num::One;
use num::rational::Ratio;

fn main() {
    let mut ratios = HashSet::new();
    for n1 in 1..10 {
        for n2 in 1..10 {
            let numerator = n1 * 10 + n2;
            for d1 in 1..10 {
                for d2 in 1..10 {
                    let denominator = d1 * 10 + d2;
                    let ratio = Ratio::new(numerator, denominator);
                    if ratio >= Ratio::one() {
                        continue;
                    }
                    if n1 == d1 && Ratio::new(n2, d2) == ratio {
                        ratios.insert(ratio);
                    }
                    if n2 == d1 && Ratio::new(n1, d2) == ratio {
                        ratios.insert(ratio);
                    }
                    if n1 == d2 && Ratio::new(n2, d1) == ratio {
                        ratios.insert(ratio);
                    }
                    if n2 == d2 && Ratio::new(n1, d1) == ratio {
                        ratios.insert(ratio);
                    }
                }
            }
        }
    }
    let ans = *ratios.iter().product::<Ratio<_>>().denom();
    dbg!(ans);
}
