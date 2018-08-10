pub fn raindrops(n: usize) -> String {
    let mut string_returned = String::from("");
    if n % 3 == 0 {
        string_returned.push_str("Pling");
    }
    if n % 5 == 0 {
        string_returned.push_str("Plang");
    }
    if n % 7 == 0 {
        string_returned.push_str("Plong");
    }
    if string_returned.len() == 0 {
        return n.to_string();
    }
    string_returned
}
