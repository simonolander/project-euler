use itertools::Itertools;

fn main() {
    let names: Vec<_> = include_str!("names.txt")
        .split(',')
        .map(|name| name.trim_matches('"'))
        .sorted()
        .collect();

    let mut ans = 0;
    for (n, &name) in names.iter().enumerate() {
        let sum: usize = name.as_bytes().iter().map(|c| c - b'A' + 1)
            .map(|c| c as usize)
            .sum();
        ans += (n + 1) * sum;
    }
    dbg!(ans);
}
