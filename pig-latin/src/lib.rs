fn translate_helper(word: &str) -> String {
  let letters: Vec<String> = word.chars().map(|letter| letter.to_string()).collect();
  let vowels = vec!["a", "e", "i", "o", "u"];
  let first_letter = &word[0..1];
  let first_two_letters = &word[0..2];

  if vowels.contains(&first_letter) || first_two_letters == "xr" || first_two_letters == "yt" {
    return word.to_string() + "ay";
  }
  let mut first_vowel_position = letters
    .iter()
    .position(|letter| vowels.contains(&letter.as_str()))
    .unwrap();
  if letters[first_vowel_position - 1] == "q" && letters[first_vowel_position] == "u" {
    first_vowel_position += 1;
  }
  let prefix = &word[0..first_vowel_position];
  let suffix = &word[first_vowel_position..];

  suffix.to_string() + prefix + "ay"
}

pub fn translate(text: &str) -> String {
  text
    .split_whitespace()
    .map(translate_helper)
    .collect::<Vec<_>>()
    .join(" ")
}
