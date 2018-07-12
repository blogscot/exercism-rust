mod card;
mod hand;

use hand::*;

pub fn winning_hands<'a>(hands: &[&'a str]) -> Option<Vec<&'a str>> {
    if hands.len() == 1 {
        return Some(hands.to_vec());
    }
    let hand1 = Hand::from(hands[0]);
    let hand2 = Hand::from(hands[1]);
    if hand1 > hand2 {
        Some(hands[0..1].to_vec())
    } else {
        Some(hands[1..].to_vec())
    }
}
