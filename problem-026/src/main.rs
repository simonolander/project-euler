fn main() {
    let limit: usize = 1000;
    let mut max_d = 0;
    let mut max_n = 0;
    for d in 2..limit {
        let mut p = 1;
        for n in 1..d {
            p *= 10;
            p %= d;
            if p == 1 {
                if n > max_n {
                    max_n = n;
                    max_d = d;
                }
                break;
            }
        }
    }

    println!("ans = {}", max_d);
}
