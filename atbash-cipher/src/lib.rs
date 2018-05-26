
pub fn encode(text: &str) -> String {
    text.to_lowercase()
        .chars()
        .filter(|&ch| ch.is_alphanumeric() && ch.is_ascii())
        .map(|ch| convert(ch).to_string())
        .collect::<Vec<_>>()
        .chunks(5)
        .map(|block| block.join("") )
        .collect::<Vec<_>>()
        .join(" ")
}

pub fn decode(text: &str) -> String {
    text.replace(" ", "")
        .chars()
        .map(convert)
        .collect()
}

fn convert(chr: char) -> char {
    let (a, z, c) = (b'a', b'z', chr as u8);

    match chr {
        'a'...'z' => (z - c + a) as char,
        _ => chr,
    }
}
