use std::sync::mpsc;
use std::sync::mpsc::Sender;
use std::{collections::HashMap, thread};

pub type LetterCount = HashMap<char, u32>;

pub fn frequency<'a>(text: &'a [&'static str], num_workers: usize) -> LetterCount {
  if text.len() < 128 {
    return frequency_helper(text);
  }
  let (tx, rx) = mpsc::channel();

  let chunks = build_chunks(text, num_workers)
    .into_iter()
    .filter(|chunk| !chunk.is_empty())
    .collect::<Vec<_>>();
  let num_chunks = chunks.len();

  for chunk in chunks {
    let tx1 = Sender::clone(&tx);
    thread::spawn(move || tx1.send(frequency_helper(&chunk)).unwrap());
  }

  let mut results = vec![];
  for _ in 0..num_chunks {
    results.push(rx.recv().unwrap());
  }

  results
    .into_iter()
    .fold(HashMap::new(), |acc, value| add(acc, value))
}

fn build_chunks<'a>(text: &[&'static str], num_workers: usize) -> Vec<Vec<&'a str>> {
  let mut chunks: Vec<Vec<&str>> = Vec::new();
  for _ in 0..num_workers {
    chunks.push(Vec::new());
  }
  let mut index = 0;
  for line in text.iter() {
    chunks[index].push(line);
    index = (index + 1) % num_workers;
  }
  chunks
}

fn add(mut list1: LetterCount, list2: LetterCount) -> LetterCount {
  for (key, value) in list2 {
    let mut counter = list1.entry(key).or_insert(0);
    *counter += value;
  }
  list1
}

pub fn frequency_helper(texts: &[&str]) -> LetterCount {
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
