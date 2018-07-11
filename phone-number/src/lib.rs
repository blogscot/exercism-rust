extern crate regex;

use regex::Regex;

pub fn number(text: &str) -> Option<String> {
  let re = Regex::new(
    r"(?x)                      # Free Spacing Mode
  ^[+1]?1?\s*[(]?
  (?P<area>[2-9]\d{2})[)]?      # Area
  \s*[-.]*
  (?P<exchange>[2-9]\d{2})      # Exchange
  \s*[-.]*
  (?P<subscriber>\d{4})         # Subscriber
  \s*$",
  ).unwrap();

  match re.captures(text) {
    Some(caps) => {
      let area = &caps["area"];
      let exchange = &caps["exchange"];
      let subscriber = &caps["subscriber"];
      Some(format!("{}{}{}", area, exchange, subscriber))
    }
    _ => None,
  }
}
