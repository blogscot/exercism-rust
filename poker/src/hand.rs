use card::{Card, Rank};
use std::cmp::Ordering;
use std::collections::HashMap;

type CardGroups = HashMap<u8, Vec<Card>>;

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
enum HandType {
  HighCard,
  OnePair,
  TwoPair,
  ThreeOfAKind,
  Straight,
  Flush,
  FullHouse,
  FourOfAKind,
  StraightFlush,
}

use hand::HandType::*;

#[derive(Debug, PartialEq)]
pub struct Hand<'a> {
  pub text: &'a str,
  cards: Vec<Card>,
  groups: Vec<Group>,
  singles: Vec<u8>,
  hand_type: HandType,
}

#[derive(Debug, PartialEq, Eq, Ord)]
struct Group(HandType, u8);

impl PartialOrd for Group {
  fn partial_cmp(&self, other: &Group) -> Option<Ordering> {
    Some(self.0.cmp(&other.0).then(self.1.cmp(&other.1)))
  }
}

impl<'a> PartialOrd for Hand<'a> {
  fn partial_cmp(&self, other: &Hand) -> Option<Ordering> {
    Some(
      self
        .hand_type
        .cmp(&other.hand_type)
        .then(self.groups.cmp(&other.groups))
        .then(self.singles.cmp(&other.singles)),
    )
  }
}

fn find_groups(cards: &[Card]) -> CardGroups {
  let empty_map: CardGroups = HashMap::new();
  cards.iter().cloned().fold(empty_map, |mut acc, card| {
    let rank = card.to_u8();
    {
      let contents = acc.entry(rank).or_insert_with(Vec::new);
      contents.push(card);
    }
    acc
  })
}

fn is_straight_helper(cards: &[Card]) -> bool {
  let mut index = 0;
  let mut value = cards[index].to_u8();
  while index < 4 {
    index += 1;
    value += 1;
    if cards[index].to_u8() != value {
      return false;
    }
  }
  true
}

fn is_straight(cards: &mut [Card]) -> bool {
  cards.sort();
  let mut test_cards = cards.to_vec();
  if is_straight_helper(&cards) {
    return true;
  }
  let ace_high_position = test_cards.iter().position(|card| card.rank == Rank::Ace);
  match ace_high_position {
    None => false,
    Some(position) => {
      test_cards[position].rank = Rank::N(1);
      test_cards.sort();
      if is_straight_helper(&test_cards) {
        cards[position].rank = Rank::N(1);
        cards.sort();
        true
      } else {
        false
      }
    }
  }
}

fn is_flush(cards: &[Card]) -> bool {
  cards.iter().skip(1).all(|card| card.suit == cards[0].suit)
}

fn classify(mut cards: &mut [Card]) -> HandType {
  let test = match (is_straight(&mut cards), is_flush(cards)) {
    (true, true) => Some(StraightFlush),
    (true, false) => Some(Straight),
    (false, true) => Some(Flush),
    _ => None,
  };
  if let Some(result) = test {
    return result;
  }
  let group: Vec<usize> = find_groups(&cards)
    .values()
    .map(|group| group.len())
    .filter(|x| *x > 1)
    .collect();
  match group.as_slice() {
    &[] => HighCard,
    &[2] => OnePair,
    &[2, 2] => TwoPair,
    &[3] => ThreeOfAKind,
    &[2, 3] => FullHouse,
    &[3, 2] => FullHouse,
    &[4] => FourOfAKind,
    unknown => panic!("{:?}", unknown),
  }
}

impl<'a> From<&'a str> for Hand<'a> {
  fn from(text: &'a str) -> Self {
    let mut cards: Vec<Card> = text.split_whitespace().map(|text| text.into()).collect();
    let hand_type = classify(&mut cards);
    let groups = find_groups(&cards);
    let (groups, singles): (CardGroups, CardGroups) =
      groups.into_iter().partition(|(_, values)| values.len() > 1);

    let (groups, singles): (Vec<Group>, Vec<u8>) = match hand_type {
      Straight | Flush | StraightFlush => {
        let singles: Vec<u8> = cards
          .iter()
          .cloned()
          .map(|card| card.to_u8())
          .rev()
          .collect();
        (Vec::new(), singles)
      }
      _ => {
        let mut groups: Vec<Group> = groups
          .into_iter()
          .map(|(key, mut cards)| Group(classify(&mut cards), key))
          .collect();
        let mut singles: Vec<u8> = singles.keys().cloned().collect();
        groups.sort();
        singles.sort();
        groups = groups.into_iter().rev().collect();
        singles = singles.into_iter().rev().collect();
        (groups, singles)
      }
    };

    Hand {
      text,
      cards,
      groups,
      singles,
      hand_type,
    }
  }
}
