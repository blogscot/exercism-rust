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

pub struct Allergies {
  allergies: Vec<Allergen>,
}

impl Allergies {
  fn get_allergens() -> Vec<Allergen> {
    vec![
      Eggs,
      Peanuts,
      Shellfish,
      Strawberries,
      Tomatoes,
      Chocolate,
      Pollen,
      Cats,
    ]
  }
  pub fn new(input: u32) -> Self {
    let allergens = Self::get_allergens();
    let mut allergies = vec![];
    for allergen in allergens {
      let value = 1u32.rotate_left(allergen as u32);
      if value & input > 0 {
        allergies.push(allergen);
      }
    }
    Allergies { allergies }
  }
  pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
    self.allergies.contains(allergen)
  }

  pub fn allergies(&self) -> &[Allergen] {
    self.allergies.as_slice()
  }
}
