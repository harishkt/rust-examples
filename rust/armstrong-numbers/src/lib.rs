pub fn is_armstrong_number(num: u32) -> bool {
    let to_str = num.to_string();
    let length = to_str.len();
    const RADIX: u32 = 10;
    let result = to_str
        .chars()
        .map(|c| c.to_digit(RADIX).unwrap().pow(length as u32))
        .sum::<u32>();
    num == result
}
