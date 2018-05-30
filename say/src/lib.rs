const MILLION: u64 = 1_000_000;
const BILLION: u64 = 1_000_000_000;
const TRILLION: u64 = 1_000_000_000_000;
const QUADRILLION: u64 = 1_000_000_000_000_000;
const QUINTILLION: u64 = 1_000_000_000_000_000_000;

pub fn encode(mut n: u64) -> String {
  let english_numbers: &[(u64, &str)] = &[
    (1, "one"),
    (2, "two"),
    (3, "three"),
    (4, "four"),
    (5, "five"),
    (6, "six"),
    (7, "seven"),
    (8, "eight"),
    (9, "nine"),
    (10, "ten"),
    (11, "eleven"),
    (12, "twelve"),
    (13, "thirteen"),
    (14, "fourteen"),
    (15, "fifteen"),
    (16, "sixteen"),
    (17, "seventeen"),
    (18, "eighteen"),
    (19, "nineteen"),
    (20, "twenty"),
    (30, "thirty"),
    (40, "forty"),
    (50, "fifty"),
    (60, "sixty"),
    (70, "seventy"),
    (80, "eighty"),
    (90, "ninety"),
    (100, "hundred"),
    (1_000, "thousand"),
    (MILLION, "million"),
    (BILLION, "billion"),
    (TRILLION, "trillion"),
    (QUADRILLION, "quadrillion"),
    (QUINTILLION, "quintillion"),
  ];

  if n == 0 {
    return "zero".to_string();
  }

  fn process(units: u64, n: &mut u64, text: &str) -> String {
    let mut output = String::new();
    let new_units = *n / units;
    output = output + &encode(new_units) + " ";
    *n -= new_units * units;
    output += text;
    if *n > 0 {
      output += " ";
    }
    output
  }

  let mut output = String::new();
  for (value, text) in english_numbers.iter().rev() {
    if n >= *value {
      if n >= QUINTILLION {
        output += &process(QUINTILLION, &mut n, text);
      } else if n >= QUADRILLION {
        output += &process(QUADRILLION, &mut n, text);
      } else if n >= TRILLION {
        output += &process(TRILLION, &mut n, text);
      } else if n >= BILLION {
        output += &process(BILLION, &mut n, text);
      } else if n >= MILLION {
        output += &process(MILLION, &mut n, text);
      } else if n >= 1000 {
        output += &process(1000, &mut n, text);
      } else if n >= 100 {
        output += &process(100, &mut n, text);
      } else if n > 20 && n <= 90 {
        output = output + text + "-";
        n -= value;
      } else {
        output += text;
        n -= value;
      }
    }
  }
  output
}
