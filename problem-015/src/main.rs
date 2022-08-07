fn main() {
    let mut answer: u128 = (21..=40).product();
    answer /= (1..=20).product::<u128>();
    dbg!(answer);
}
