fn find_factors(n: u64, mut factors: Vec<u64>) -> Vec<u64> {
  if n < 2 {
    return factors;
  }

  let range = (2..).take_while(|x| x * x <= n);
  for value in range {
    if n % value == 0 {
      factors.push(value);
      return find_factors(n / value, factors);
    }
  }
  factors.push(n);
  factors
}

pub fn factors(n: u64) -> Vec<u64> {
  find_factors(n, Vec::new())
}
