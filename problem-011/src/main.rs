use std::cmp::max;

fn main() {
    let grid: Vec<Vec<u64>> = include_str!("input.txt")
        .lines()
        .map(|line| line.split(' ').map(|s| s.parse().unwrap()).collect())
        .collect();
    let mut product = 0;
    for r in 0..grid.len() {
        for c in 0..grid[r].len() {
            let mut horizontal_product = 1;
            let mut vertical_product = 1;
            let mut diagonal_product = 1;
            let mut diagonal_product_2 = 1;
            for offset in 0..4 {
                horizontal_product *= grid
                    .get(r)
                    .and_then(|row| row.get(c + offset))
                    .unwrap_or(&0);
                vertical_product *= grid
                    .get(r + offset)
                    .and_then(|row| row.get(c))
                    .unwrap_or(&0);
                diagonal_product *= grid
                    .get(r + offset)
                    .and_then(|row| row.get(c + offset))
                    .unwrap_or(&0);
                diagonal_product_2 *= grid
                    .get(r.overflowing_sub(offset).0)
                    .and_then(|row| row.get(c + offset))
                    .unwrap_or(&0);
            }
            product = max(product, horizontal_product);
            product = max(product, vertical_product);
            product = max(product, diagonal_product);
            product = max(product, diagonal_product_2);
        }
    }
    dbg!(product);
}
