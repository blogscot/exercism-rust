use std::collections::HashMap;

pub fn word_count(text: &str) -> HashMap<String, u32> {
    let cleaned = clean(&text.to_lowercase());
    let mut words = HashMap::new();

    for word in cleaned.split_terminator(' ') {
        let counter = words.entry(word.into()).or_insert(0);
        *counter += 1;
    }
    words
}

fn clean(text: &str) -> String {
    let text = text.chars()
        .filter(|&ch| ch.is_alphanumeric() || ch == ' ')
        .collect::<String>();
    text.replace("  ", " ")
}
