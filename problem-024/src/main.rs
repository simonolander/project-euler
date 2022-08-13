fn main() {
    let mut remainder: usize = 999_999;
    let mut digits: Vec<usize> = Vec::new();
    let mut remaining_digits: Vec<usize> = Vec::from_iter(0..=9);
    let mut fac: usize = (1..=10).product();
    while !remaining_digits.is_empty() {
        fac /= remaining_digits.len();
        let index = remainder / fac;
        let digit = remaining_digits[index];
        digits.push(digit);
        remaining_digits.remove(index);
        remainder -= index * fac;
    }
    let ans: String = digits.iter().map(|&n| n.to_string()).collect();
    dbg!(ans);
}
