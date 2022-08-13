fn main() {
    let lim = 1001;
    let mut ans: usize = 1;
    let mut n: usize = 1;
    let mut k = 2;
    while n < lim * lim {
        for _ in 0..4 {
            n += k;
            ans += n;
        }
        k += 2;
    }

    dbg!(ans);
}
