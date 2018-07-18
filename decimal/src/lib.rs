extern crate num_bigint;

use num_bigint::{BigInt, Sign};
use std::cmp::Ordering;
use std::fmt;
use std::ops::{Add, Mul, Sub};

#[derive(Clone, PartialEq)]
pub struct Decimal {
  value: BigInt,
  point_position: u32,
}

impl PartialOrd for Decimal {
  fn partial_cmp(&self, other: &Decimal) -> Option<Ordering> {
    let mut first = self.clone();
    let mut second = other.clone();
    level_decimals(&mut first, &mut second);
    Some(first.value.cmp(&second.value))
  }
}

impl Decimal {
  pub fn try_from(input: &str) -> Option<Decimal> {
    let sign = match input.chars().nth(0) {
      Some('-') => Sign::Minus,
      _ => Sign::Plus,
    };
    let digits: Vec<u8> = input
      .chars()
      .filter(|num| num.is_digit(10))
      .map(|chr| chr as u8 - 48)
      .collect();
    let point_position = input
      .chars()
      .filter(|chr| chr.is_digit(10) || *chr == '.')
      .position(|chr| chr == '.')
      .unwrap_or_else(|| digits.len()) as u32;
    let point_position = digits.len() as u32 - point_position;
    let value = BigInt::from_radix_be(sign, &digits, 10).unwrap();
    let mut output = Decimal {
      value,
      point_position,
    };
    normalise(&mut output);
    Some(output)
  }
}

impl fmt::Debug for Decimal {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(
      f,
      "{{ value: {} point_position: {} }}",
      self.value.to_str_radix(10),
      self.point_position
    )
  }
}

/**
 * Adjusts the number of trailing zeros,
 * e.g. 1000 with 3 decimal places is converted to 1.0.
 */
fn normalise(num: &mut Decimal) {
  if num.point_position == 0 {
    return;
  }
  if num.value == BigInt::from(0) {
    num.point_position = 0;
  } else {
    while num.value.clone() % 10 == BigInt::from(0) {
      num.value = num.value.clone() / 10;
      num.point_position -= 1;
    }
  }
}

/**
 * Modifies one of the decimals such that they use the same
 * number of decimal places.
 */
fn level_decimals(decimal1: &mut Decimal, decimal2: &mut Decimal) {
  let position1 = decimal1.point_position;
  let position2 = decimal2.point_position;
  match position1.cmp(&position2) {
    Ordering::Equal => (),
    Ordering::Greater => {
      let mut difference = position1 - position2;
      while difference > 0 {
        decimal2.value *= 10;
        difference -= 1;
      }
      decimal2.point_position = position1;
    }
    Ordering::Less => {
      let mut difference = position2 - position1;
      while difference > 0 {
        decimal1.value *= 10;
        difference -= 1;
      }
      decimal1.point_position = position2;
    }
  }
}

impl Add for Decimal {
  type Output = Decimal;

  fn add(self, other: Decimal) -> Decimal {
    let mut first = self.clone();
    let mut second = other.clone();
    level_decimals(&mut first, &mut second);
    let mut output = Decimal {
      value: first.value + second.value,
      point_position: first.point_position,
    };
    normalise(&mut output);
    output
  }
}

impl Sub for Decimal {
  type Output = Decimal;

  fn sub(self, other: Decimal) -> Decimal {
    let mut first = self.clone();
    let mut second = other.clone();
    level_decimals(&mut first, &mut second);
    let mut output = Decimal {
      value: first.value - second.value,
      point_position: first.point_position,
    };
    normalise(&mut output);
    output
  }
}

impl Mul for Decimal {
  type Output = Decimal;

  fn mul(self, other: Decimal) -> Decimal {
    let mut first = self.clone();
    let mut second = other.clone();
    level_decimals(&mut first, &mut second);
    let mut output = Decimal {
      value: first.value * second.value,
      point_position: first.point_position + second.point_position,
    };
    normalise(&mut output);
    output
  }
}
