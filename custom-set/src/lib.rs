#[derive(Debug, PartialEq)]
pub struct CustomSet<T> {
  values: Vec<T>,
}

impl<T> CustomSet<T>
where
  T: Clone + PartialEq + Ord,
{
  pub fn new(input: &[T]) -> Self {
    let mut values = input.to_vec();
    values.sort();
    values.dedup();
    CustomSet { values }
  }
  pub fn is_empty(&self) -> bool {
    self.values.is_empty()
  }
  pub fn contains(&self, value: &T) -> bool {
    self.values.as_slice().contains(&value)
  }
  pub fn is_subset(&self, other: &Self) -> bool {
    if self.values.is_empty() || self.values == other.values {
      return true;
    }
    let set2 = other.values.as_slice();
    self
      .values
      .iter()
      .all(|set1_value| set2.contains(set1_value))
  }
  pub fn is_disjoint(&self, other: &Self) -> bool {
    if self.values.is_empty() {
      return true;
    }
    let set2 = other.values.as_slice();
    self
      .values
      .iter()
      .all(|set1_value| !set2.contains(set1_value))
  }
  pub fn add(&mut self, new_value: T) {
    self.values.push(new_value);
    self.values.sort();
    self.values.dedup();
  }
  pub fn intersection(&self, other: &Self) -> Self {
    let common = CustomSet::new(&[]);
    let set2 = other.values.as_slice();
    self
      .values
      .iter()
      .cloned()
      .fold(common, |mut acc, set1_value| {
        if set2.contains(&set1_value) {
          acc.add(set1_value);
          acc
        } else {
          acc
        }
      })
  }
  pub fn difference(&self, other: &Self) -> Self {
    let common = CustomSet::new(&[]);
    let set2 = other.values.as_slice();
    self
      .values
      .iter()
      .cloned()
      .fold(common, |mut acc, set1_value| {
        if !set2.contains(&set1_value) {
          acc.add(set1_value);
          acc
        } else {
          acc
        }
      })
  }
  pub fn union(&self, other: &Self) -> Self {
    let mut output = self.values.clone();
    output.extend(other.values.clone());
    output.sort();
    output.dedup();
    CustomSet { values: output }
  }
}
