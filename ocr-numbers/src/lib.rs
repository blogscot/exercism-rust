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
  check_valid_line_count(&input)?;
  check_valid_column_length(&input)?;

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
  let ocr_0to9_encodings = vec![
    125_820, 756, 123_606, 123_390, 11_691, 129_222, 129_465, 118_854, 130_194, 129_951,
  ];

  let encoding = encode(input);
  let find_encoding = ocr_0to9_encodings
    .iter()
    .position(|&value| value == encoding);
  match find_encoding {
    Some(position) => Ok(position.to_string()),
    _ => Ok("?".to_string()),
  }
}

fn encode(input: &str) -> u32 {
  let base3_encoding: String = input
    .chars()
    .filter(|&chr| chr != '\n')
    .map(|chr| match chr {
      '_' => "2",
      '|' => "1",
      _ => "0",
    })
    .collect();
  u32::from_str_radix(&base3_encoding, 3).unwrap()
}

fn check_valid_line_count(input: &str) -> Result<(), Error> {
  let line_count = input
    .chars()
    .fold(0usize, |acc, chr| if chr == '\n' { acc + 1 } else { acc });
  if line_count % 3 != 0 {
    Err(Error::InvalidRowCount(line_count + 1))
  } else {
    Ok(())
  }
}

fn check_valid_column_length(input: &str) -> Result<(), Error> {
  let segment: String = input.chars().take_while(|&chr| chr != '\n').collect();
  let segment_length = segment.len();
  if segment_length % 3 != 0 {
    Err(Error::InvalidColumnCount(segment_length))
  } else {
    Ok(())
  }
}
