fn main() {
    let mut year = 1900;
    let mut month = 10;
    let mut day = 1;
    let mut day_of_week = 1;
    let mut ans = 0;
    loop {
        if year == 2001 {
            break
        }
        if day == 1 && day_of_week == 7 {
            ans += 1;
            dbg!(year, month, day);
        }
        day += 1;
        day_of_week += 1;
        if day_of_week == 8 {
            day_of_week = 1
        }
        let last_day_of_month = match month {
            1 => 31,
            2 if year % 4 == 0 => 29,
            2 => 28,
            3 => 31,
            4 => 30,
            5 => 31,
            6 => 30,
            7 => 31,
            8 => 31,
            9 => 30,
            10 => 31,
            11 => 30,
            12 => 31,
            _ => panic!()
        };
        if day > last_day_of_month {
            day = 1;
            month += 1;
            if month > 12 {
                month = 1;
                year += 1;
            }
        }
    }
    dbg!(ans);
}
