extern crate regex;

use regex::Regex;

pub fn number(text: &str) -> Option<String> {
  let re = Regex::new(
    r"^[+1]?1?\s*[(]?(?P<area>[2-9]\d{2})[)]?[ -.]*(?P<exchange>[2-9]\d{2})[ -.]*(?P<subscriber>\d{4})\s*$",
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
