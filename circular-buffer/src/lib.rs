use std::collections::VecDeque;

#[derive(Debug, PartialEq)]
pub enum Error {
  EmptyBuffer,
  FullBuffer,
}

pub struct CircularBuffer<T> {
  size: usize,
  buffer: VecDeque<T>,
}

impl<T> CircularBuffer<T> {
  pub fn new(size: usize) -> Self {
    CircularBuffer {
      size,
      buffer: VecDeque::with_capacity(size),
    }
  }
  fn is_buffer_full(&self) -> bool {
    self.buffer.len() >= self.size
  }
  pub fn read(&mut self) -> Result<T, Error> {
    self.buffer.pop_front().ok_or(Error::EmptyBuffer)
  }
  pub fn write(&mut self, value: T) -> Result<(), Error> {
    if self.is_buffer_full() {
      return Err(Error::FullBuffer);
    }
    self.buffer.push_back(value);
    Ok(())
  }
  pub fn clear(&mut self) {
    self.buffer.clear();
  }
  pub fn overwrite(&mut self, value: T) {
    if self.is_buffer_full() {
      self.buffer.pop_front();
    }
    self.buffer.push_back(value);
  }
}
