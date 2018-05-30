fn is_even(x: u64) -> bool {
  x % 2 == 0
}

fn collatz_helper(n: u64, count: u64) -> u64 {
  match n {
    1 => count,
    n if is_even(n) => collatz_helper(n / 2, count + 1),
    _ => collatz_helper(n * 3 + 1, count + 1),
  }
}

pub fn collatz(n: u64) -> Option<u64> {
  match n {
    0 => None,
    _ => Some(collatz_helper(n, 0)),
  }
}
