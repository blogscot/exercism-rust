type Position = (i8, i8);

enum Direction {
  Right,
  Down,
  Left,
  Up,
}

struct Matrix {
  moves_taken: u8,
  moves_remaining: u8,
  position: Position,
  direction: Direction,
}

/**
 * [
 * [ (0,0) (0,1) (0,2) ],
 * [ (1,0) (1,1) (1,2) ],
 * [ (2,0) (2,1) (2,2) ],
 * ]
*/

impl Matrix {
  fn new(length: usize) -> Self {
    Matrix {
      moves_taken: 0,
      moves_remaining: length as u8,
      position: (0, -1),
      direction: Direction::Right,
    }
  }
  fn move_right(&mut self) {
    self.position.1 += 1
  }
  fn move_left(&mut self) {
    self.position.1 -= 1
  }
  fn move_up(&mut self) {
    self.position.0 -= 1
  }
  fn move_down(&mut self) {
    self.position.0 += 1
  }
}

impl Iterator for Matrix {
  type Item = Position;

  fn next(&mut self) -> Option<Position> {
    if self.moves_remaining <= 0 {
      return None;
    };
    match self.direction {
      Direction::Right => {
        self.move_right();
        self.moves_taken += 1;
        if self.moves_taken == self.moves_remaining {
          self.direction = Direction::Down;
          self.moves_taken = 0;
          self.moves_remaining -= 1;
        }
        Some(self.position)
      }
      Direction::Left => {
        self.move_left();
        self.moves_taken += 1;
        if self.moves_taken == self.moves_remaining {
          self.direction = Direction::Up;
          self.moves_taken = 0;
          self.moves_remaining -= 1;
        }
        Some(self.position)
      }
      Direction::Up => {
        self.move_up();
        self.moves_taken += 1;
        if self.moves_taken == self.moves_remaining {
          self.direction = Direction::Right;
          self.moves_taken = 0;
        }
        Some(self.position)
      }
      Direction::Down => {
        self.move_down();
        self.moves_taken += 1;
        if self.moves_taken == self.moves_remaining {
          self.direction = Direction::Left;
          self.moves_taken = 0;
        }
        Some(self.position)
      }
    }
  }
}

pub fn spiral_matrix(size: usize) -> Vec<Vec<usize>> {
  let mut spiral_matrix = vec![];
  let mut row: Vec<usize> = vec![];
  for _ in 0..size {
    row.push(0);
  }
  for _ in 0..size {
    spiral_matrix.push(row.clone());
  }

  let matrix = Matrix::new(size);
  for (index, (row, col)) in matrix.enumerate() {
    spiral_matrix[row as usize][col as usize] = index + 1;
  }
  spiral_matrix
}
