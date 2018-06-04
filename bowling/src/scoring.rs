pub enum Frame {
  Open(u16, u16),
  Spare(u16),
  Strike,
  FinalWithBonus(u16, u16, u16),
}

use self::Frame::*;

fn get_next_roll(current_position: usize, game: &[Frame]) -> u16 {
  let next_position = current_position + 1;
  match game[next_position] {
    Open(pins, _) => pins,
    Spare(pins) => pins,
    Strike => 10,
    FinalWithBonus(pins, _, _) => pins,
  }
}

fn get_next_two_rolls(current_position: usize, game: &[Frame]) -> u16 {
  let next_position = current_position + 1;
  match game[next_position] {
    Open(first, second) => first + second,
    Spare(_) => 10,
    Strike => 10 + get_next_roll(next_position, game),
    FinalWithBonus(first, second, _) => first + second,
  }
}

pub fn score(game: &[Frame]) -> Option<u16> {
  let mut total = 0;
  for (index, frame) in game.iter().enumerate() {
    let frame_score = match frame {
      Open(first, second) => first + second,
      Spare(_) => 10 + get_next_roll(index, game),
      Strike => 10 + get_next_two_rolls(index, game),
      FinalWithBonus(first, second, bonus) => first + second + bonus,
    };
    total += frame_score
  }
  Some(total)
}

#[cfg(test)]
mod tests {

  use super::*;

  #[test]
  fn game_with_only_open_frames() {
    let game = vec![
      Open(6, 3),
      Open(6, 3),
      Open(6, 3),
      Open(6, 3),
      Open(6, 3),
      Open(6, 3),
      Open(6, 3),
      Open(6, 3),
      Open(6, 3),
      Open(6, 3),
    ];
    assert_eq!(Some(90), score(&game));
  }

  #[test]
  fn game_with_single_spare() {
    let game = vec![
      Spare(6),
      Open(6, 3),
      Open(6, 3),
      Open(6, 3),
      Open(6, 3),
      Open(6, 3),
      Open(6, 3),
      Open(6, 3),
      Open(6, 3),
      Open(6, 3),
    ];
    assert_eq!(Some(97), score(&game));
  }

  #[test]
  fn get_next_rolls() {
    let game = vec![
      Spare(6),
      Open(6, 3),
      Strike,
      Open(6, 3),
      Open(6, 3),
      Spare(8),
      Open(6, 3),
      Open(6, 3),
      Spare(9),
      Open(6, 3),
    ];
    assert_eq!(6, get_next_roll(0, &game));
    assert_eq!(10, get_next_roll(1, &game));
    assert_eq!(6, get_next_roll(2, &game));
    assert_eq!(6, get_next_roll(3, &game));
    assert_eq!(8, get_next_roll(4, &game));
    assert_eq!(6, get_next_roll(5, &game));
    assert_eq!(6, get_next_roll(6, &game));
    assert_eq!(9, get_next_roll(7, &game));
    assert_eq!(6, get_next_roll(8, &game));
  }

  #[test]
  fn test_get_next_two_rolls() {
    let game = vec![
      Spare(6),
      Open(6, 3),
      Open(6, 3),
      Strike,
      Strike,
      Spare(8),
      Open(6, 3),
      Open(6, 3),
      Spare(9),
      Open(6, 3),
    ];
    assert_eq!(9, get_next_two_rolls(0, &game));
    assert_eq!(9, get_next_two_rolls(1, &game));
    assert_eq!(20, get_next_two_rolls(2, &game));
    assert_eq!(18, get_next_two_rolls(3, &game));
    assert_eq!(10, get_next_two_rolls(4, &game));
    assert_eq!(9, get_next_two_rolls(5, &game));
    assert_eq!(9, get_next_two_rolls(6, &game));
    assert_eq!(10, get_next_two_rolls(7, &game));
    assert_eq!(9, get_next_two_rolls(8, &game));
  }

  #[test]
  fn game_with_multiple_spares() {
    let game = vec![
      Spare(6),   // 16
      Open(6, 3), // 25
      Open(6, 3), // 34
      Open(6, 3), // 43
      Open(6, 3), // 52
      Spare(8),   // 68
      Open(6, 3), // 77
      Open(6, 3), // 86
      Spare(9),   // 102
      Open(6, 3), // 111
    ];
    assert_eq!(Some(111), score(&game));
  }

  #[test]
  fn game_with_multiple_strikes_and_spares() {
    let game = vec![
      Spare(4),   // 13
      Open(3, 6), // 22
      Open(6, 1), // 29
      Open(7, 2), // 38
      Strike,     // 58
      Spare(8),   // 74
      Open(6, 3), // 83
      Strike,     // 103
      Spare(9),   // 119
      Open(6, 3), // 128
    ];
    assert_eq!(Some(128), score(&game));
  }

  #[test]
  fn a_perfect_game() {
    let game = vec![
      Strike,                     // 30
      Strike,                     // 60
      Strike,                     // 90
      Strike,                     // 120
      Strike,                     // 150
      Strike,                     // 180
      Strike,                     // 210
      Strike,                     // 240
      Strike,                     // 270
      FinalWithBonus(10, 10, 10), // 300
    ];
    assert_eq!(Some(300), score(&game));
  }

  #[test]
  fn final_frame_contains_bonus() {
    let game = vec![
      Open(0, 0),
      Open(0, 0),
      Open(0, 0),
      Open(0, 0),
      Open(0, 0),
      Open(0, 0),
      Open(0, 0),
      Open(0, 0),
      Open(0, 0),
      FinalWithBonus(7, 3, 6),
    ];
    assert_eq!(Some(16), score(&game));
  }

}
