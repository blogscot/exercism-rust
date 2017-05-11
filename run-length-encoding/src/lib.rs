pub fn encode(text: &str) -> String {
    if text.is_empty() {return text.into()}

    let mut encoded = String::new();
    let mut count: u32 = 1;
    let mut current = text.chars().nth(0).unwrap();

    for c in text.chars().skip(1) {
        if c == current {
            count += 1;
        } else {
            encoded.push_str(&convert(count));
            encoded.push(current);
            current = c;
            count = 1;
        }
    }
    encoded.push_str(&convert(count));
    encoded.push(current);

    encoded
}

fn convert(n: u32) -> String {
    match n {
        1 => "".into(),
        _ => n.to_string(),
    }
}


pub fn decode(text: &str) -> String {
    let mut numerals: String = String::new();
    let mut decoded: String = String::new();

    for c in text.chars() {
        if c.is_numeric() {
            numerals.push(c);
        } else {
            let num: usize = numerals.parse().unwrap_or(1);
            decoded.push_str(&c.to_string().repeat(num));
            numerals.clear();
        }
    }
    decoded
}
