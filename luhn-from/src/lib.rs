#[derive(Debug)]
pub struct Luhn {
  text: String,
}

impl<T: ToString> From<T> for Luhn {
  fn from(input: T) -> Self {
    Luhn {
      text: input.to_string(),
    }
  }
}

impl Luhn {
  pub fn is_valid(&self) -> bool {
    let cleaned = self.text.replace(" ", "");
    let is_valid_sequence = |seq: &str| cleaned.len() > 1 && seq.chars().all(char::is_numeric);

    if is_valid_sequence(&cleaned) {
      return Self::calculate_sum(&cleaned) % 10 == 0;
    }
    false
  }

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
}
