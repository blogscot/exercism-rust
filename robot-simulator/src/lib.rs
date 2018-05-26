#[derive(PartialEq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

use Direction::*;

pub struct Robot {
    x: isize,
    y: isize,
    direction: Direction,
}

impl Robot {
    pub fn new(x: isize, y: isize, d: Direction) -> Self {
        Robot {
            x: x,
            y: y,
            direction: d,
        }
    }

    pub fn turn_right(mut self) -> Self {
        self.direction = match self.direction {
            North => East,
            East => South,
            South => West,
            West => North,
        };
        self
    }

    pub fn turn_left(mut self) -> Self {
        self.direction = match self.direction {
            North => West,
            West => South,
            South => East,
            East => North,
        };
        self
    }

    pub fn advance(mut self) -> Self {
        match self.direction {
            North => self.y += 1,
            West => self.x += -1,
            South => self.y += -1,
            East => self.x += 1,
        };
        self
    }

    pub fn instructions(mut self, instructions: &str) -> Self {
        for instruction in instructions.chars() {
            self = match instruction {
                'L' => self.turn_left(),
                'R' => self.turn_right(),
                'A' => self.advance(),
                _ => self, // Contemplate killing all stupid humans
            }
        }
        self
    }

    pub fn position(&self) -> (isize, isize) {
        (self.x, self.y)
    }

    pub fn direction(&self) -> &Direction {
        &self.direction
    }
}
