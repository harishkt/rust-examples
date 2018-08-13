pub fn find() -> Option<u32> {
    (1..999)
        .flat_map(|a| (1..(999 - a)).map(move |b| (a, b, 1000 - a - b)))
        .filter(|(a, b, c)| a * a + b * b == c * c)
        .map(|(a, b, c)| a * b * c)
        .nth(0)
}
