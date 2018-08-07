use std::char;

const ADJACENCIES: [(i8, i8); 8] = [
  (-1, -1),
  (-1, 0),
  (-1, 1),
  (0, -1),
  (0, 1),
  (1, -1),
  (1, 0),
  (1, 1),
];

pub fn annotate(input: &[&str]) -> Vec<String> {
  if input.is_empty() {
    return vec![];
  }
  let board = Board::new(input);
  board
    .data
    .iter()
    .enumerate()
    .map(|(pos_0, row)| {
      row
        .iter()
        .enumerate()
        .map(|(pos_1, elem)| {
          if *elem != '*' {
            let count = board.neighbours((pos_0 as u8, pos_1 as u8));
            if count > 0 {
              char::from_digit(u32::from(count), 10).unwrap()
            } else {
              ' '
            }
          } else {
            *elem
          }
        })
        .collect::<String>()
    })
    .collect()
}

#[derive(Debug)]
struct Board {
  width: u8,
  height: u8,
  data: Vec<Vec<char>>,
}

impl Board {
  fn new(input: &[&str]) -> Self {
    let data = input
      .into_iter()
      .map(|row| row.chars().collect::<Vec<_>>())
      .collect::<Vec<_>>();
    let height = data.len() as u8;
    let width = data[0].len() as u8;
    Board {
      width,
      height,
      data,
    }
  }
  fn neighbours(&self, position: (u8, u8)) -> u8 {
    let position = (position.0 as i8, position.1 as i8);
    ADJACENCIES
      .iter()
      .map(|adjacent| (position.0 + adjacent.0, position.1 + adjacent.1))
      .filter(|neighbour| {
        neighbour.0 >= 0
          && neighbour.1 >= 0
          && neighbour.0 < self.height as i8
          && neighbour.1 < self.width as i8
      })
      .fold(0, |acc, pos| {
        if self.data[pos.0 as usize][pos.1 as usize] == '*' {
          acc + 1
        } else {
          acc
        }
      })
  }
}
