pub fn series(digits: &str, len: usize) -> Vec<String> {
  let mut output = vec![];
  if digits.len() < len {
    return output;
  }

  for start in 0..=(digits.len() - len) {
    let substr: &str = &digits[start..(start + len)];
    output.push(substr.to_string());
  }
  output
}
