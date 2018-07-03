fn get_template_string(chr: char) -> (String, Vec<String>) {
    let alphabet: Vec<String> = "ABCDEFGHIJKLMNOPQRSTUVWXYZ"
        .chars()
        .map(|letter| letter.to_string())
        .collect();
    let letter_position = alphabet
        .iter()
        .position(|letter| letter == &chr.to_string())
        .unwrap();
    let forward_pattern = &alphabet[0..=letter_position];
    let reverse_pattern: Vec<String> = forward_pattern.iter().cloned().skip(1).rev().collect();
    let pattern = reverse_pattern.join("").to_string() + &forward_pattern.join("");
    (pattern, forward_pattern.to_vec())
}

pub fn get_diamond(letter: char) -> Vec<String> {
    let (template, forward_pattern) = get_template_string(letter);
    let mut top = vec![];
    for letter in forward_pattern {
        let filtered: String = template
            .chars()
            .map(move |chr| {
                let chr = chr.to_string();
                if chr == letter {
                    chr
                } else {
                    " ".to_string()
                }
            })
            .collect();
        top.push(filtered);
    }
    let mut diamond = top.clone();
    let bottom: Vec<String> = top.iter().cloned().rev().skip(1).collect();
    diamond.extend(bottom);
    diamond
}
