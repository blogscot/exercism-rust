pub fn reply(words: &str) -> &str {
    let is_shouting = |s: &str| s.to_uppercase() == s;

    match words {
        words if words.is_empty() => "Fine. Be that way!",
        words if words.ends_with('?') => "Sure.",
        words if is_shouting(words) => "Whoa, chill out!",
        _ => "Whatever.",
    }
}
