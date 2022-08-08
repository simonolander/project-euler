use std::cmp::max;

fn main() {
    let mut pyramid: Vec<Vec<usize>> = include_str!("input.txt")
        .lines()
        .map(parse_line)
        .collect();
    for r in (1..pyramid.len()).rev() {
        let lower_row = pyramid[r].clone();
        let upper_row = &mut pyramid[r-1];
        for c in 0..upper_row.len() {
            upper_row[c] += max(lower_row[c], lower_row[c + 1]);
        }
    }
    let answer = pyramid[0][0];
    dbg!(answer);
}

fn parse_line(line: &str) -> Vec<usize> {
    line.split(' ')
        .map(|s| s.parse().unwrap())
        .collect()
}