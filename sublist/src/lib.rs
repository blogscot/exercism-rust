#[derive(Debug, PartialEq)]
pub enum Comparison {
  Equal,
  Unequal,
  Sublist,
  Superlist,
}

use Comparison::*;

fn is_infix_of<T: Copy + Clone + PartialEq>(first: &[T], second: &[T]) -> bool {
  if first.is_empty() {
    return true;
  }
  if second.is_empty() {
    return false;
  }

  let clipped: Vec<T> = second
    .iter()
    .cloned()
    .skip_while(|&value| value != first[0])
    .collect();

  if clipped.is_empty() || first.len() > clipped.len() {
    return false;
  }

  let mut zipped = first.iter().zip(clipped.iter());
  if !zipped.all(|(a, b)| a == b) {
    return is_infix_of(first, &clipped[1..]);
  }
  true
}

pub fn sublist<T: Copy + Clone + PartialEq>(first: &[T], second: &[T]) -> Comparison {
  if first == second {
    return Equal;
  } else if is_infix_of(first, second) {
    return Sublist;
  } else if is_infix_of(second, first) {
    return Superlist;
  } else {
    return Unequal;
  }
}
