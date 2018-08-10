pub fn reply(message: &str) -> &str {
    if message.trim_right().ends_with("?") {
        if message.replace("?", "")
            .chars().all(|c: char| c.is_uppercase() || c.is_whitespace() ) {
            return "Calm down, I know what I'm doing!";
        }
        return "Sure.";
    } else if message.chars().all(|c: char| c.is_whitespace())  {
        return "Fine. Be that way!";
    } else if message.chars()
        .all(|c: char| c.is_uppercase() || !c.is_alphabetic() ) {
        return "Whoa, chill out!";
    } else {
        return "Whatever."
    }
}
