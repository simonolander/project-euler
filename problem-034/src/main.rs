fn main() {
    let limit_digits = (1..)
        .into_iter()
        .find(|&c| fac(9) * c < 10u32.pow(c - 1))
        .unwrap();
    let limit = 10u32.pow(limit_digits);
    let ans: u32 = (3..limit).into_iter()
        .filter(|&n| digits(n).iter().copied().map(fac).sum::<u32>() == n)
        .sum();
    dbg!(ans);
}

fn digits(n: u32) -> Vec<u32> {
    match n {
        0 => vec![0],
        mut n => {
            let mut ds = Vec::new();
            while n != 0 {
                ds.push(n % 10);
                n /= 10;
            }
            ds
        }
    }
}

fn fac(n: u32) -> u32 {
    match n {
        0 => 1,
        _ => n * fac(n - 1)
    }
}
