#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
  Abundant,
  Perfect,
  Deficient,
}

use Classification::*;

pub fn classify(num: u64) -> Option<Classification> {
  if num == 0 {
    return None;
  }
  let sum: u64 = (1..num).filter(|value| num % value == 0).sum();
  match sum {
    sum if sum == num => Some(Perfect),
    sum if sum > num => Some(Abundant),
    _ => Some(Deficient),
  }
}
