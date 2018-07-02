use bucket::*;
use std::fmt;

#[derive(Clone, PartialEq)]
pub struct Buckets {
  left: Bucket,
  right: Bucket,
}

impl Buckets {
  pub fn new(left_capacity: u8, right_capacity: u8) -> Self {
    let left = Bucket::new(left_capacity);
    let right = Bucket::new(right_capacity);
    Buckets { left, right }
  }
  pub fn get_contents(&self) -> (u8, u8) {
    (self.left.contents, self.right.contents)
  }
  pub fn contains(&self, contents: u8) -> bool {
    self.left.contents == contents || self.right.contents == contents
  }
  pub fn set_contents(&mut self, contents: (u8, u8)) -> Self {
    let mut buckets = self.clone();
    let (left, right) = contents;
    buckets.left.contents = left;
    buckets.right.contents = right;
    buckets
  }
  pub fn fill_left(&self) -> Self {
    let mut buckets = self.clone();
    buckets.left.contents = self.left.capacity;
    buckets
  }
  pub fn fill_right(&self) -> Self {
    let mut buckets = self.clone();
    buckets.right.contents = self.right.capacity;
    buckets
  }
  pub fn empty_left(&self) -> Self {
    let mut buckets = self.clone();
    buckets.left.contents = 0;
    buckets
  }
  pub fn empty_right(&self) -> Self {
    let mut buckets = self.clone();
    buckets.right.contents = 0;
    buckets
  }
  pub fn transfer_right(&self) -> Self {
    let (from, to) = Self::transfer(&self.left, &self.right);
    Buckets {
      left: to,
      right: from,
    }
  }
  pub fn transfer_left(&self) -> Self {
    let (from, to) = Self::transfer(&self.right, &self.left);
    Buckets {
      left: from,
      right: to,
    }
  }
  fn transfer(from: &Bucket, to: &Bucket) -> (Bucket, Bucket) {
    let mut from = from.clone();
    let mut to = to.clone();

    let maxiumum_possible = to.capacity - to.contents;
    if from.contents < maxiumum_possible {
      to.contents += from.contents;
      from.contents = 0;
    } else {
      to.contents = to.capacity;
      from.contents -= maxiumum_possible;
    }
    (to, from)
  }
}

impl fmt::Debug for Buckets {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "({} {})", self.left.contents, self.right.contents)
  }
}
