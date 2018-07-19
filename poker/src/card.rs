use std::{cmp::Ordering, fmt};

#[derive(Clone, Debug, PartialEq, Hash, PartialOrd, Eq, Ord)]
pub enum Suit {
  Clubs,
  Diamonds,
  Hearts,
  Spades,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Hash, Eq, Ord)]
pub enum Rank {
  N(u8),
  Jack,
  Queen,
  King,
  Ace,
}

#[derive(PartialEq, Clone, Hash, Eq, Ord)]
pub struct Card {
  pub rank: Rank,
  pub suit: Suit,
}

impl Card {
  pub fn to_u8(&self) -> u8 {
    match self.rank {
      Rank::N(n) => n,
      Rank::Jack => 11,
      Rank::Queen => 12,
      Rank::King => 13,
      Rank::Ace => 14,
    }
  }
}

impl PartialOrd for Card {
  fn partial_cmp(&self, other: &Card) -> Option<Ordering> {
    Some(self.rank.cmp(&other.rank))
  }
}

impl<'a> Into<Card> for &'a str {
  fn into(self) -> Card {
    let mut as_vector: Vec<char> = self.chars().collect();
    let suit = as_vector.pop().unwrap();
    let rank = as_vector.into_iter().nth(0).unwrap();

    let rank = match rank {
      '2'...'9' => Rank::N(rank.to_digit(10).unwrap() as u8),
      '1' => Rank::N(10u8),
      'J' => Rank::Jack,
      'Q' => Rank::Queen,
      'K' => Rank::King,
      'A' => Rank::Ace,
      value => panic!("Invalid value: {}", value),
    };
    let suit = match suit {
      'C' => Suit::Clubs,
      'D' => Suit::Diamonds,
      'H' => Suit::Hearts,
      'S' => Suit::Spades,
      suit => panic!("Invalid suit: {}", suit),
    };
    Card { rank, suit }
  }
}

impl fmt::Debug for Card {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "<{:?} of {:?}>", self.rank, self.suit)
  }
}
