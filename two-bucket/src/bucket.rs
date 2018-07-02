#[derive(Clone, Debug, PartialEq)]
pub struct Bucket {
  pub contents: u8,
  pub capacity: u8,
}

impl Bucket {
  pub fn new(capacity: u8) -> Self {
    Bucket {
      contents: 0,
      capacity,
    }
  }
}
