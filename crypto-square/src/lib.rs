fn calculate_length(value: usize) -> usize {
  let mut columns = 0;
  while columns * columns < value {
    columns += 1;
  }
  columns
}

fn pad_string(vector: &mut Vec<char>, length: usize) {
  while vector.len() % length != 0 {
    vector.push(' ');
  }
}

pub fn encrypt(input: &str) -> String {
  if input.is_empty() {
    return "".into();
  }
  let mut filtered: Vec<char> = input
    .to_lowercase()
    .chars()
    .filter(|letter| letter.is_alphanumeric())
    .collect();

  let column_length = calculate_length(filtered.len());
  pad_string(&mut filtered, column_length);
  let chunked: Vec<_> = filtered.chunks(column_length).collect();

  let mut transposed = Vec::new();
  for index in 0..column_length {
    let mut new_string = String::new();
    for item in &chunked {
      new_string.push(item[index]);
    }
    transposed.push(new_string);
  }
  transposed.join(" ")
}
