pub fn nth(n: usize) -> Option<u32> {
  match n {
    0 => None,
    _ => (1..).filter(|&x| is_prime(x)).nth(n - 1),
  }
}

pub fn is_prime(n: u32) -> bool {
  if n <= 1 {
    return false;
  }
  let mut range = (2..=n).take_while(|x| x * x <= n);
  !range.any(|x| n % x == 0)
}
