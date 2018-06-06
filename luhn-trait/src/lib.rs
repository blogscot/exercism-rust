#[derive(Debug)]
pub struct Luhn {
  text: String,
}

pub trait ValidLuhn {
  fn valid_luhn(&self) -> bool;
  fn calculate_sum(sequence: &str) -> u32 {
    let is_odd = |x| x % 2 != 0;

    sequence
      .chars()
      .rev()
      .enumerate()
      .map(|(index, chr)| {
        let mut value = chr.to_digit(10).unwrap();

        if is_odd(index) {
          value *= 2;
          if value > 9 {
            value -= 9;
          }
        }
        value
      })
      .sum()
  }
  fn is_valid_sequence(seq: &str) -> bool {
    seq.len() > 1 && seq.chars().all(char::is_numeric)
  }
}

impl ValidLuhn for String {
  fn valid_luhn(&self) -> bool {
    let cleaned = self.replace(" ", "");
    if Self::is_valid_sequence(&cleaned) {
      return Self::calculate_sum(&cleaned) % 10 == 0;
    }
    false
  }
}

impl ValidLuhn for &'static str {
  fn valid_luhn(&self) -> bool {
    self.to_string().valid_luhn()
  }
}

impl ValidLuhn for u8 {
  fn valid_luhn(&self) -> bool {
    self.to_string().valid_luhn()
  }
}

impl ValidLuhn for u16 {
  fn valid_luhn(&self) -> bool {
    self.to_string().valid_luhn()
  }
}
impl ValidLuhn for u32 {
  fn valid_luhn(&self) -> bool {
    self.to_string().valid_luhn()
  }
}
impl ValidLuhn for u64 {
  fn valid_luhn(&self) -> bool {
    self.to_string().valid_luhn()
  }
}
impl ValidLuhn for usize {
  fn valid_luhn(&self) -> bool {
    self.to_string().valid_luhn()
  }
}
