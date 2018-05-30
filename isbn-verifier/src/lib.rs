/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
  let reversed: String = isbn.chars().rev().collect();
  let (head, tail) = reversed.split_at(1);
  let check_digit = match head {
    "X" => 10,
    digit => match digit.parse::<u32>() {
      Ok(value) => value,
      _ => return false,
    },
  };
  let mut count = 2;
  let mut total = check_digit;

  let filtered = tail.to_string().replace("-", "");

  for character in filtered.chars() {
    match character.to_digit(10) {
      Some(value) => {
        total += count * value;
        count += 1;
      }
      _ => return false,
    }
  }
  total % 11 == 0
}
