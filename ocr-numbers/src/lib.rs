#[derive(Debug, PartialEq)]
pub enum Error {
  InvalidRowCount(usize),
  InvalidColumnCount(usize),
}

pub fn convert(input: &str) -> Result<String, Error> {
  let mut output = vec![];
  for line in input.lines().collect::<Vec<_>>().chunks(4) {
    output.push(convert_line(&line.join("\n"))?);
  }
  Ok(output.join(","))
}

fn convert_line(input: &str) -> Result<String, Error> {
  if let Some(count) = is_valid_line_count(&input) {
    return Err(Error::InvalidRowCount(count));
  }
  if let Some(count) = is_valid_column_length(&input) {
    return Err(Error::InvalidColumnCount(count));
  }

  let mut parts = vec![];
  for line in input.split('\n') {
    for chunk in line.chars().collect::<Vec<_>>().chunks(3) {
      parts.push(chunk.iter().collect::<String>());
    }
  }

  let mut individual_digit_parts = vec![];
  let num_digits = (parts.len() + 1) / 4;
  for x in 0..num_digits {
    for y in 0..4 {
      let index = (y * num_digits) + x;
      individual_digit_parts.push(parts[index].clone());
    }
  }

  let mut assembled_digits = vec![];
  for parts in individual_digit_parts.chunks(4) {
    assembled_digits.push(parts.join("\n"));
  }

  let result = assembled_digits
    .iter()
    .flat_map(|digit| convert_digit(&digit))
    .collect::<String>();

  Ok(result)
}

fn convert_digit(input: &str) -> Result<String, Error> {
  let binary_encodings = vec![
    125820, 756, 123606, 123390, 11691, 129222, 129465, 118854, 130194, 129951,
  ];

  let encoding = encode(input);
  let find_encoding = binary_encodings.iter().position(|&value| value == encoding);
  match find_encoding {
    Some(position) => Ok(position.to_string()),
    _ => Ok("?".to_string()),
  }
}

fn encode(input: &str) -> u32 {
  let result: String = input
    .chars()
    .filter(|&chr| chr != '\n')
    .map(|chr| match chr {
      '_' => "2",
      '|' => "1",
      _ => "0",
    })
    .collect();
  u32::from_str_radix(&result, 3).unwrap()
}

fn is_valid_line_count(input: &str) -> Option<usize> {
  let count = input
    .chars()
    .fold(0usize, |acc, chr| if chr == '\n' { acc + 1 } else { acc });
  if count % 3 != 0 {
    Some(count + 1)
  } else {
    None
  }
}

fn is_valid_column_length(input: &str) -> Option<usize> {
  let segment: String = input.chars().take_while(|&chr| chr != '\n').collect();
  if segment.len() % 3 != 0 {
    Some(segment.len())
  } else {
    None
  }
}
