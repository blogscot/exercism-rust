extern crate itertools;
extern crate permutohedron;

use itertools::Itertools;
use permutohedron::Heap;
use std::collections::HashMap;

struct Formula {
  equation_lhs: Vec<String>,
  equation_rhs: String,
  letters: Vec<char>,
  num_letters: usize,
  first_letters: Vec<char>,
  legend: HashMap<char, u32>,
}

impl Formula {
  pub fn new(text: &str) -> Self {
    let mut equation: Vec<String> = text.split("==").map(|str| str.trim().to_string()).collect();
    let equation_rhs = equation.pop().unwrap().to_string();
    let equation_lhs = equation[0]
      .split('+')
      .map(|str| str.trim().to_string())
      .collect::<Vec<_>>();
    let mut letters: Vec<char> = text.chars().filter(|chr| chr.is_alphabetic()).collect();
    let first_letters: Vec<char> = equation_lhs
      .iter()
      .map(|word| word.chars().nth(0).unwrap())
      .collect();
    letters.sort();
    letters.dedup();
    let num_letters = letters.len();
    let mut legend = HashMap::new();
    for &letter in &letters {
      legend.insert(letter, 0);
    }

    Formula {
      equation_lhs,
      equation_rhs,
      letters,
      num_letters,
      first_letters,
      legend,
    }
  }
  fn match_found(&self) -> bool {
    let lhs_result = self.evaluate_list(&self.equation_lhs);
    let rhs = self.evaluate(&self.equation_rhs);

    // reject a rhs result that has a leading zero
    let solution_length = rhs.to_string().len();
    let expected_num_letters = self.equation_rhs.len();

    lhs_result == rhs && solution_length == expected_num_letters
  }
  fn update_legend(&mut self, digits: &[usize]) {
    let letters: Vec<char> = self.letters.clone();
    letters
      .into_iter()
      .zip(digits.into_iter())
      .for_each(|(key, digit)| {
        let _ = self.legend.insert(key, *digit as u32);
      });
  }
  fn first_letters_contain_zero(&self) -> bool {
    self
      .first_letters
      .iter()
      .any(|letter| &self.legend[letter] == &0)
  }
  fn evaluate(&self, text: &str) -> u32 {
    let digits: Vec<String> = text
      .chars()
      .map(|chr| {
        let digit = *&self.legend[&chr];
        digit.to_string()
      })
      .collect();
    digits.join("").parse::<u32>().unwrap()
  }
  fn evaluate_list(&self, variables: &[String]) -> u32 {
    variables
      .iter()
      .map(|variable| self.evaluate(variable))
      .sum()
  }
  fn find_match(&mut self) -> Option<HashMap<char, u8>> {
    let letters_length = self.num_letters;
    for mut digit_set in (0..10).combinations(letters_length) {
      let heap = Heap::new(&mut digit_set);
      for permutation in heap {
        self.update_legend(&permutation);
        if self.first_letters_contain_zero() {
          continue;
        }
        if self.match_found() {
          let letters_iter = self.letters.clone().into_iter();
          let pairs: HashMap<char, u8> = letters_iter
            .zip(permutation.iter())
            .map(|(letter, digit)| (letter, *digit as u8))
            .collect();
          return Some(pairs);
        }
      }
    }
    None
  }
}
pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
  let mut formula = Formula::new(input);
  formula.find_match()
}
