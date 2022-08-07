fn main() {
    let answer: usize = (1..=1000)
        .map(count)
        .sum();
    dbg!(answer);
}

fn count(n: i32) -> usize {
    match n {
        0 => 0,
        1 => "one".len(),
        2 => "two".len(),
        3 => "three".len(),
        4 => "four".len(),
        5 => "five".len(),
        6 => "six".len(),
        7 => "seven".len(),
        8 => "eight".len(),
        9 => "nine".len(),
        10 => "ten".len(),
        11 => "eleven".len(),
        12 => "twelve".len(),
        13 => "thirteen".len(),
        14 => "fourteen".len(),
        15 => "fifteen".len(),
        16 => "sixteen".len(),
        17 => "seventeen".len(),
        18 => "eighteen".len(),
        19 => "nineteen".len(),
        20..=29 => "twenty".len() + count(n % 10),
        30..=39 => "thirty".len() + count(n % 10),
        40..=49 => "forty".len() + count(n % 10),
        50..=59 => "fifty".len() + count(n % 10),
        60..=69 => "sixty".len() + count(n % 10),
        70..=79 => "seventy".len() + count(n % 10),
        80..=89 => "eighty".len() + count(n % 10),
        90..=99 => "ninety".len() + count(n % 10),
        100..=999 if n % 100 == 0 => count(n / 100) + "hundred".len(),
        100..=999 => count(n / 100) + "hundred".len() + "and".len() + count(n % 100),
        1000 => "one".len() + "thousand".len(),
        _ => panic!(),
    }
}
