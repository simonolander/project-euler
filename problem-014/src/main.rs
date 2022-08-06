use std::collections::HashMap;

fn main() {
    let mut memo = HashMap::new();
    for n in 1..1_000_000 {
        collatz(n, &mut memo);
    }
    let max = memo.iter().max_by_key(|&e| e.1).unwrap().0;
    dbg!(max);
}

fn collatz(n: u64, memo: &mut HashMap<u64, u64>) -> u64 {
    if memo.contains_key(&n) {
        return memo[&n];
    }

    let answer = if n == 1 {
        1
    } else if n % 2 == 0 {
        collatz(n / 2, memo) + 1
    } else {
        collatz(3 * n + 1, memo) + 1
    };

    memo.insert(n, answer);
    answer
}
