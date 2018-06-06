pub fn is_valid(sequence: &str) -> bool {
  let cleaned = sequence.replace(" ", "");
  if cleaned.len() > 1 && is_valid_sequence(&cleaned) {
    return calculate_sum(&cleaned) % 10 == 0;
  }
  false
}

fn calculate_sum(sequence: &str) -> u32 {
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
    .sum::<u32>()
}

fn is_valid_sequence(sequence: &str) -> bool {
  sequence.chars().all(char::is_numeric)
}

fn is_odd(x: usize) -> bool {
  x % 2 != 0
}
