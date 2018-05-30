pub fn is_armstrong_number(num: u32) -> bool {
  let stringified = num.to_string();
  let len = stringified.len();
  let value: u32 = stringified
    .chars()
    .map(|chr| chr.to_digit(10).unwrap().pow(len as u32))
    .sum();
  value == num
}
