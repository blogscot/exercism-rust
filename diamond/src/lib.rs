fn get_template_string(letter: char) -> (String, Vec<String>) {
    let letter_as_string = letter.to_string();
    let alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let alphabet: Vec<String> = alphabet.chars().map(|letter| letter.to_string()).collect();
    let letter_position = alphabet
        .iter()
        .position(|ref letter| letter == &&letter_as_string)
        .unwrap();
    let forward_pattern = &alphabet[0..=letter_position];
    let reverse_pattern: Vec<String> = forward_pattern.iter().cloned().skip(1).rev().collect();
    let pattern = reverse_pattern.join("").to_string() + &forward_pattern.join("");
    (pattern, forward_pattern.to_vec())
}

pub fn get_diamond(letter: char) -> Vec<String> {
    let (template, forward_pattern) = get_template_string(letter);
    let mut output = vec![];
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
        output.push(filtered);
    }
    let mut diamond = output.clone();
    let diamond_bottom: Vec<String> = output.iter().cloned().rev().skip(1).collect();
    diamond.extend(diamond_bottom);
    diamond
}
