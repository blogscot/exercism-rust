use rayon::{prelude::*, ThreadPoolBuilder};
use std::collections::HashMap;

pub fn frequency_helper(line: &str) -> HashMap<char, usize> {
  let mut map = HashMap::new();

  for chr in line.chars().filter(|c| c.is_alphabetic()) {
    if let Some(c) = chr.to_lowercase().next() {
      (*map.entry(c).or_insert(0)) += 1;
    }
  }
  map
}

fn add(mut list1: HashMap<char, usize>, list2: HashMap<char, usize>) -> HashMap<char, usize> {
  for (key, value) in list2 {
    let mut counter = list1.entry(key).or_insert(0);
    *counter += value;
  }
  list1
}

pub fn frequency(texts: &[&str], num_workers: usize) -> HashMap<char, usize> {
  if num_workers == 1 {
    texts
      .iter()
      .map(|text| frequency_helper(text))
      .fold(HashMap::new(), |acc, value| add(acc, value))
  } else {
    ThreadPoolBuilder::new().num_threads(num_workers);
    texts
      .par_iter()
      .map(|text| frequency_helper(text))
      .reduce_with(add)
      .unwrap_or(HashMap::new())
  }
}
