
pub fn reply(words : &str) -> &str {
    match words {
        words if words.ends_with("?") => "Sure.",
        words if is_shouting(words)   => "Whoa, chill out!",
        words if words.is_empty()     => "Fine. Be that way!",
        _                             => "Whatever.",
    }
}

fn is_shouting(s : &str) -> bool {
    s.to_uppercase() == s && s.to_lowercase() != s
}
