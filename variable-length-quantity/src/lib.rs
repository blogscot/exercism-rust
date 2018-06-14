#[derive(Debug, PartialEq)]
pub enum Error {
  IncompleteNumber,
  Overflow,
}

const MAX_FIRST_BLOCK: u8 = 0x8f;
const VLQ_END_MARKER: u8 = 0x7f;
const MAX_BLOCKS: usize = 5;
const U8_BIT_MASK: u8 = 0x7f;
const U32_BIT_MASK: u32 = 0x7f;
const BIT_EIGHT: u8 = 0x80;
const SEVEN_BITS: u32 = 7;

/// Convert a list of numbers to a stream of bytes encoded with variable length encoding.
pub fn to_bytes(values: &[u32]) -> Vec<u8> {
  values
    .into_iter()
    .flat_map(|value| to_bytes_helper(&[*value]))
    .collect()
}

fn to_bytes_helper(values: &[u32]) -> Vec<u8> {
  let mut num: u32 = values[0];
  if num == 0 {
    return vec![0u8];
  }
  let mut octets = vec![];
  let mut first_octet = true;

  while num > 0 {
    if first_octet {
      octets.push(num as u8 & U8_BIT_MASK);
      first_octet = false;
    } else {
      octets.push(num as u8 | BIT_EIGHT);
    }
    num >>= SEVEN_BITS;
  }
  octets.reverse();
  octets
}

/// Given a stream of bytes, extract all numbers which are encoded in there.
pub fn from_bytes(values: &[u8]) -> Result<Vec<u32>, Error> {
  if !values.is_empty() && *values.last().unwrap() > VLQ_END_MARKER {
    return Err(Error::IncompleteNumber);
  }

  let mut result = vec![];
  let mut blocks = vec![];
  for &value in values {
    if value > VLQ_END_MARKER {
      blocks.push(value);
    } else {
      blocks.push(value);
      if blocks.len() >= MAX_BLOCKS && *blocks.first().unwrap() > MAX_FIRST_BLOCK {
        return Err(Error::Overflow);
      }
      result.push(from_bytes_helper(&blocks));
      blocks = vec![];
    }
  }
  let flattened: Vec<u32> = result.into_iter().flat_map(|x| x).collect();
  Ok(flattened)
}

pub fn from_bytes_helper(bytes: &[u8]) -> Vec<u32> {
  let mut shift_left = 0;
  let mut result: u32 = 0;
  let mut octets: Vec<u8> = bytes.to_vec();
  octets.reverse();
  for octet in octets {
    let mut tmp = octet as u32 & U32_BIT_MASK;
    tmp <<= shift_left;
    shift_left += SEVEN_BITS;
    result += tmp;
  }
  vec![result]
}
