pub fn is_pangram(words: &str) -> bool {
    (b'a' .. b'z'+1).all(|letter| words.to_lowercase().contains(letter as char))
}

