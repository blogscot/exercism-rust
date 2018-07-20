extern crate rand;

use rand::prelude::*;

#[derive(Default)]
pub struct Robot {
  name: String,
}

fn get_random_letter() -> char {
  let alphabet: Vec<_> = (b'A'..=b'Z').collect();
  let letters = alphabet.as_slice();
  let mut rng = thread_rng();
  letters[rng.gen_range(0, 26)] as char
}

fn get_random_number() -> u8 {
  thread_rng().gen_range(0, 10)
}

impl Robot {
  pub fn new() -> Robot {
    let mut name = String::default();
    for _ in 0..2 {
      name += &get_random_letter().to_string();
    }
    for _ in 0..3 {
      name += &get_random_number().to_string();
    }
    Robot { name }
  }

  pub fn name(&self) -> &str {
    &self.name
  }

  pub fn reset_name(&mut self) {
    let new_robot = Self::new();
    self.name = new_robot.name;
  }
}
