pub fn is_leap_year(year: i32) -> bool {
    match year % 4 {
        0 => match year % 100 == 0 && year % 400 != 0 {
            true => false,
            false => true
        },
        _ => false
    }
}
