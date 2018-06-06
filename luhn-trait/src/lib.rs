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

impl<T: ToString> ValidLuhn for T {
  fn valid_luhn(&self) -> bool {
    let cleaned: String = self.to_string().replace(" ", "");
    if Self::is_valid_sequence(&cleaned) {
      return Self::calculate_sum(&cleaned) % 10 == 0;
    }
    false
  }
}
