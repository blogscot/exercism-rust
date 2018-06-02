extern crate rand;

use rand::prelude::*;

fn rotate_char(chr: char, number: u8) -> char {
  let (a, c) = (b'a', chr as u8);

  match chr {
    'a'...'z' => (((c - a + number) % 26) + a) as char,
    _ => chr,
  }
}

fn calc_distance(chr: char) -> u8 {
  chr as u8 - b'a'
}

fn is_valid(text: &str) -> bool {
  !text.is_empty()
    && text.chars().all(|chr| chr.is_alphabetic())
    && text.to_ascii_lowercase() == text
}

fn generate_random_key(length: usize) -> String {
  let alphabet: Vec<_> = (b'a'..=b'z').collect();
  let letters = alphabet.as_slice();
  let mut rng = thread_rng();
  let random_key: String = (0..length)
    .map(|_| letters[rng.gen_range(0, 26)] as char)
    .collect();
  random_key
}

pub fn encode(key: &str, s: &str) -> Option<String> {
  if !is_valid(key) || !is_valid(s) {
    return None;
  }
  let res = s
    .chars()
    .zip(key.chars().cycle())
    .map(|(chr, key)| rotate_char(chr, calc_distance(key)))
    .collect();
  Some(res)
}

pub fn decode(key: &str, s: &str) -> Option<String> {
  if !is_valid(key) || !is_valid(s) {
    return None;
  }
  let res = s
    .chars()
    .zip(key.chars().cycle())
    .map(|(chr, key)| rotate_char(chr, 26 - calc_distance(key)))
    .collect();
  Some(res)
}

pub fn encode_random(s: &str) -> (String, String) {
  let random_key = generate_random_key(128);
  match encode(&random_key, s) {
    Some(encoded) => (random_key, encoded),
    None => (random_key, "".into()),
  }
}
