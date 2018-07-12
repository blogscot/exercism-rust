mod card;
mod hand;

use hand::*;

pub fn winning_hands<'a>(hands: &[&'a str]) -> Option<Vec<&'a str>> {
  if hands.len() == 1 {
    return Some(hands.to_vec());
  }
  let player_hands: Vec<Hand> = hands.iter().map(|&text| Hand::from(text)).collect();
  let mut index = 0;
  let first = player_hands.first().unwrap();
  for (new_index, player_hand) in player_hands.iter().enumerate() {
    if player_hand > first {
      index = new_index;
    }
  }
  Some(hands[index..index + 1].to_vec())
}
