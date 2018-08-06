use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a [&str]) -> HashSet<&'a str> {
  possible_anagrams
    .iter()
    .cloned()
    .filter(|&anagram| anagram.to_lowercase() != word.to_lowercase() && compare(anagram, word))
    .collect()
}

fn compare(left: &str, right: &str) -> bool {
  let mut right = right
    .chars()
    .map(|chr| chr.to_lowercase().to_string())
    .collect::<Vec<_>>();
  let mut left = left
    .chars()
    .map(|chr| chr.to_lowercase().to_string())
    .collect::<Vec<_>>();
  left.sort();
  right.sort();
  left == right
}
