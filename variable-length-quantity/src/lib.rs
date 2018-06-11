#[derive(Debug, PartialEq)]
pub enum Error {
  IncompleteNumber,
  Overflow,
}

pub fn to_bytes_helper(values: &[u32]) -> Vec<u32> {
  let input = values[0];
  if input <= 127 {
    return values.into();
  }
  let binary_text = format!("{:01$b}", input, 8);
  let blocks = blockify(&binary_text);
  let prefixed: Vec<String> = blocks.iter().map(|block| "1".to_string() + block).collect();
  let result = clear_last_block_prefix(prefixed);
  result.iter().fold(Vec::new(), |mut acc, elem| {
    acc.push(to_hex(elem));
    acc
  })
}

/// Convert a list of numbers to a stream of bytes encoded with variable length encoding.
pub fn to_bytes(values: &[u32]) -> Vec<u32> {
  values
    .into_iter()
    .flat_map(|value| to_bytes_helper(&[*value]))
    .collect()
}

/// Given a stream of bytes, extract all numbers which are encoded in there.
pub fn from_bytes(bytes: &[u8]) -> Result<Vec<u32>, Error> {
  unimplemented!("Convert the list of bytes {:?} to a list of numbers", bytes)
}

fn to_hex(text: &str) -> u32 {
  u32::from_str_radix(text, 2).unwrap()
}

/// Pad string to be nicely divisible by block length.
fn pad(input: &str) -> String {
  let mut text = input.to_string();
  while text.len() % 7 != 0 {
    text = "0".to_string() + &text;
  }
  text
}

// Turn binary digits into blocks of 7 digits
fn blockify(text: &str) -> Vec<String> {
  let binary = pad(text);
  let mut input = binary.as_str();
  let mut blocks = vec![];
  loop {
    let (block, rest) = input.split_at(7);
    blocks.push(block.into());
    if rest.is_empty() {
      break;
    }
    input = rest;
  }
  blocks
}

// The final binary block is indicated by a '0' prefix.
fn clear_last_block_prefix(blocks: Vec<String>) -> Vec<String> {
  let mut blocks = blocks.clone();
  let reverse = |text: String| text.chars().rev().collect();
  let last = blocks.pop().unwrap();
  let mut reversed: String = reverse(last);
  reversed.pop();
  reversed.push('0');
  let cleared: String = reverse(reversed);
  blocks.push(cleared);
  blocks
}
