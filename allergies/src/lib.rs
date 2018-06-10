#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Allergen {
  Eggs = 0,
  Peanuts,
  Shellfish,
  Strawberries,
  Tomatoes,
  Chocolate,
  Pollen,
  Cats,
}

use Allergen::*;

static ALLERGENS: [Allergen; 8] = [
  Eggs,
  Peanuts,
  Shellfish,
  Strawberries,
  Tomatoes,
  Chocolate,
  Pollen,
  Cats,
];

pub struct Allergies {
  allergies: Vec<Allergen>,
}

impl Allergies {
  pub fn new(input: u32) -> Self {
    let mut allergies = vec![];
    for allergen in &ALLERGENS {
      let value = 1 << *allergen as u32;
      if value & input > 0 {
        allergies.push(*allergen);
      }
    }
    Allergies { allergies }
  }
  pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
    self.allergies.contains(allergen)
  }

  pub fn allergies(self) -> Vec<Allergen> {
    self.allergies
  }
}
