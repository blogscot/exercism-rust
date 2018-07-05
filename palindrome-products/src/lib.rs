pub type Palindrome = u64;

fn is_palindrome(num: u64) -> bool {
  let mut number = num;
  let mut rev = 0;
  while number > 0 {
    let digit = number % 10;
    rev = rev * 10 + digit;
    number /= 10;
  }

  num == rev
}

fn get_products(start: Palindrome, end: Palindrome) -> Vec<Palindrome> {
  let mut products = Vec::new();
  for a in start..=end {
    for b in a..=end {
      let product = a * b;
      if is_palindrome(product) && !products.contains(&product) {
        products.push(product);
      }
    }
  }
  products
}

pub fn get_palindrome_products(min: Palindrome, max: Palindrome) -> Vec<Palindrome> {
  let mut products = get_products(min, max);
  products.sort();
  products
}

pub fn min(palindromes: &[Palindrome]) -> Option<Palindrome> {
  if palindromes.is_empty() {
    return None;
  }
  palindromes.first().and_then(|value| Some(*value))
}

pub fn max(palindromes: &[Palindrome]) -> Option<Palindrome> {
  if palindromes.is_empty() {
    return None;
  }
  palindromes.last().and_then(|value| Some(*value))
}
