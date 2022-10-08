pub fn reverse(&n: &u64, &base: &u64) -> u64 {
    let mut n = n;
    let mut r = 0;
    while n != 0 {
        r *= base;
        r += n % base;
        n /= base;
    }
    return r;
}

pub fn is_palindrome(n: &u64, base: &u64) -> bool {
    return *n == reverse(n, base);
}

pub fn is_palindrome_10(n: &u64) -> bool {
    return *n == reverse(n, &10);
}

pub fn is_palindrome_2(n: &u64) -> bool {
    return *n == reverse(n, &2);
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest(n, base, expected,
    case(123, 10, 321),
    case(0, 10, 0),
    case(1, 10, 1),
    case(1001, 10, 1001),
    case(40123939, 10, 93932104),
    case(1000, 10, 1),
    case(0, 2, 0),
    case(0b101101, 2, 0b101101),
    case(0b111000, 2, 0b111),
    case(0b1011000111, 2, 0b1110001101),
    case(0xabcdef, 16, 0xfedcba),
    )]
    fn reverse_test(n: u64, base: u64, expected: u64) {
        assert_eq!(reverse(&n, &base), expected);
    }


    #[rstest(n, base, expected,
    case(123, 10, false),
    case(0, 10, true),
    case(1, 10, true),
    case(1001, 10, true),
    case(40123939, 10, false),
    case(1000, 10, false),
    case(0, 2, true),
    case(0b101101, 2, true),
    case(0b111000, 2, false),
    case(0b1011000111, 2, false),
    case(0xabcdef, 16, false),
    case(0xabbaabba, 16, true),
    )]
    fn palindrome_test(n: u64, base: u64, expected: bool) {
        assert_eq!(is_palindrome(&n, &base), expected);
    }
}
