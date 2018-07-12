use std::{cmp::Ordering, fmt};

#[derive(Clone, Debug, PartialEq, Hash, PartialOrd, Eq, Ord)]
pub enum Suit {
  Clubs,
  Diamonds,
  Hearts,
  Spades,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Hash, Eq, Ord)]
pub enum Value {
  AceLow = 1,
  Two,
  Three,
  Four,
  Five,
  Six,
  Seven,
  Eight,
  Nine,
  Ten,
  Jack,
  Queen,
  King,
  AceHigh,
}

#[derive(PartialEq, Clone, Hash, Eq, Ord)]
pub struct Card {
  pub value: Value,
  pub suit: Suit,
}

impl PartialOrd for Card {
  fn partial_cmp(&self, other: &Card) -> Option<Ordering> {
    Some(self.value.cmp(&other.value))
  }
}

impl<'a> Into<Card> for &'a str {
  fn into(self) -> Card {
    let mut as_vector: Vec<char> = self.chars().collect();
    let card_suit = as_vector.pop().unwrap();
    let card_value: String = as_vector.into_iter().collect();

    let value = match card_value.as_str() {
      "A" => Value::AceHigh,
      "2" => Value::Two,
      "3" => Value::Three,
      "4" => Value::Four,
      "5" => Value::Five,
      "6" => Value::Six,
      "7" => Value::Seven,
      "8" => Value::Eight,
      "9" => Value::Nine,
      "10" => Value::Ten,
      "J" => Value::Jack,
      "Q" => Value::Queen,
      "K" => Value::King,
      value => panic!("Invalid value: {}", value),
    };
    let suit = match card_suit {
      'C' => Suit::Clubs,
      'D' => Suit::Diamonds,
      'H' => Suit::Hearts,
      'S' => Suit::Spades,
      suit => panic!("Invalid suit: {}", suit),
    };
    Card { value, suit }
  }
}

impl fmt::Debug for Card {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "<{:?} of {:?}>", self.value, self.suit)
  }
}
