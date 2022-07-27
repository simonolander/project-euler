fn main() {
    let range = 1..=100;
    let sum_of_squares: i32 = range.clone().map(|n| n * n).sum();
    let sum = range.sum::<i32>();
    let square_of_sums = sum * sum;
    let difference = square_of_sums - sum_of_squares;
    dbg!(difference);
}
