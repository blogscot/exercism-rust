mod scoring;

use scoring::{score, Frame, Frame::*};

#[derive(Debug, PartialEq)]
pub enum Error {
  NotEnoughPinsLeft,
  GameComplete,
}

#[derive(PartialEq)]
enum NextThrow {
  First,
  Second,
  Bonus,
}

use NextThrow::*;

pub struct BowlingGame {
  bonus_throw: bool,
  first_throw: u16,
  second_throw: u16,
  pins_remaining: u16,
  frames_played: u8,
  next_throw: NextThrow,
  scores: Vec<Frame>,
}

impl BowlingGame {
  pub fn new() -> Self {
    BowlingGame {
      bonus_throw: false,
      first_throw: 0,
      second_throw: 0,
      pins_remaining: 10,
      frames_played: 0,
      next_throw: First,
      scores: vec![],
    }
  }

  pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
    let is_final_frame = self.frames_played == 9;

    if pins > self.pins_remaining {
      return Err(Error::NotEnoughPinsLeft);
    } else if self.frames_played == 10 {
      return Err(Error::GameComplete);
    }

    match self.next_throw {
      First => {
        if pins == 10 && is_final_frame {
          self.bonus_throw = true;
          self.first_throw = 10;
          self.pins_remaining = 10;
          self.next_throw = Second;
        } else if pins == 10 {
          self.scores.push(Strike);
          self.first_throw = 0;
          self.pins_remaining = 10;
          self.next_throw = First;
          self.frames_played += 1;
        } else {
          self.first_throw = pins;
          self.pins_remaining -= pins;
          self.next_throw = Second;
        }
      }
      Second => {
        let total = self.first_throw + pins;
        if total == 10 && is_final_frame {
          self.bonus_throw = true;
        }

        if self.bonus_throw {
          self.second_throw = pins;
          if pins == self.pins_remaining {
            self.pins_remaining = 10;
          } else {
            self.pins_remaining -= pins;
          }
          self.next_throw = Bonus;
        } else if total == 10 {
          self.scores.push(Spare(self.first_throw));
          self.first_throw = 0;
          self.next_throw = First;
          self.frames_played += 1;
          self.pins_remaining = 10;
        } else {
          self.scores.push(Open(self.first_throw, pins));
          self.first_throw = 0;
          self.next_throw = First;
          self.frames_played += 1;
          self.pins_remaining = 10;
        }
      }
      Bonus => {
        if pins > self.pins_remaining {
          return Err(Error::NotEnoughPinsLeft);
        }
        self
          .scores
          .push(FinalWithBonus(self.first_throw, self.second_throw, pins));
        self.frames_played += 1;
      }
    }
    Ok(())
  }

  pub fn score(&self) -> Option<u16> {
    if self.frames_played == 10 {
      score(&self.scores)
    } else {
      None
    }
  }
}
