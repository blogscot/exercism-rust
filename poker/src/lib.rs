mod card;
mod hand;

use hand::*;
use std::cmp::Ordering;

pub fn winning_hands<'a>(hands: &[&'a str]) -> Option<Vec<&'a str>> {
  let mut player_hands: Vec<Hand> = hands.iter().map(|&text| Hand::from(text)).collect();
  player_hands.sort_by(|a, b| a.partial_cmp(b).unwrap());
  player_hands.last().map(|winner| {
    player_hands
      .iter()
      .rev()
      .take_while(|&hand| hand.partial_cmp(winner) == Some(Ordering::Equal))
      .map(|hand| hand.text)
      .collect()
  })
}
