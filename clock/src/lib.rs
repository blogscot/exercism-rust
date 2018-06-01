use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Clock {
  total_time: i32,
}

impl fmt::Display for Clock {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let hours = (self.total_time / 60) % 24;
    let minutes = self.total_time % 60;
    write!(f, "{:02}:{:02}", hours, minutes)
  }
}

impl Clock {
  pub fn new(hours: i32, minutes: i32) -> Self {
    let mut total_time = hours * 60 + minutes;
    total_time = Self::normalise(total_time, 60 * 24);
    Clock { total_time }
  }
  pub fn add_minutes(&mut self, minutes: i32) -> Self {
    let mut total_time = self.total_time + minutes;
    total_time = Self::normalise(total_time, 60 * 24);
    Clock { total_time }
  }
  fn normalise(value: i32, limit: i32) -> i32 {
    (value % limit + limit) % limit
  }
}
