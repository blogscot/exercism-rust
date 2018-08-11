use std::{collections::HashMap, thread};

pub type LetterCount = HashMap<char, u32>;

pub fn frequency(text: &[&str], num_workers: usize) -> LetterCount {
  let mut handles: Vec<thread::JoinHandle<LetterCount>> = Vec::new();

  for chunk in build_chunks(text, num_workers) {
    handles.push(thread::spawn(move || frequency_helper(chunk)));
  }

  handles
    .into_iter()
    .map(|handle| handle.join().unwrap())
    .fold(HashMap::new(), |acc, value| add(&acc, &value))
}

fn build_chunks(text: &[&str], num_workers: usize) -> Vec<Vec<String>> {
  let mut chunks: Vec<Vec<String>> = Vec::new();
  for _ in 0..num_workers {
    chunks.push(Vec::new());
  }
  let mut index = 0;
  for line in text.iter() {
    chunks[index].push(line.to_string());
    index = (index + 1) % num_workers;
  }
  chunks
}

fn add(list1: &LetterCount, list2: &LetterCount) -> LetterCount {
  let mut output = list1.clone();
  for (key, value) in list2 {
    let mut counter = output.entry(*key).or_insert(0);
    *counter += value;
  }
  output
}

pub fn frequency_helper(texts: Vec<String>) -> LetterCount {
  let mut map = HashMap::new();

  for line in texts {
    for chr in line.chars().filter(|c| c.is_alphabetic()) {
      if let Some(c) = chr.to_lowercase().next() {
        (*map.entry(c).or_insert(0)) += 1;
      }
    }
  }

  map
}
