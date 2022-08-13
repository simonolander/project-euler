fn main() {
    let power = 5;
    let nine_power = 9u32.pow(power);
    let max_digits = (1..)
        .into_iter()
        .find(|&n| nine_power * n < 10u32.pow(n))
        .unwrap();
    let limit = 10u32.pow(max_digits);
    let ans: u32 = (2..limit)
        .into_iter()
        .filter(|&n| n == digits(n).iter().map(|d| d.pow(power)).sum())
        .sum();
    dbg!(ans);
}

fn digits(n: u32) -> Vec<u32> {
    let mut n = n;
    let mut digits = Vec::new();
    if n == 0 {
        digits.push(0);
    } else {
        while n != 0 {
            digits.push(n % 10);
            n /= 10;
        }
    }
    digits.reverse();
    digits
}