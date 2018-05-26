pub fn rotate(text: &str, number: u8) -> String {
    text.chars()
        .map(|ch| rotate_char(ch, number))
        .collect::<String>()
}

fn rotate_char(chr: char, number: u8) -> char {
    let (upper_a, a, c) = ('A' as u8, 'a' as u8, chr as u8);

    match chr {
        'A'...'Z' => (((c - upper_a + number) % 26) + upper_a) as char,
        'a'...'z' => (((c - a + number) % 26) + a) as char,
        _ => chr,
    }
}
