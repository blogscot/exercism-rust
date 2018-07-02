extern crate two_bucket;

use two_bucket::buckets::Buckets;

#[test]
fn create_a_pair_of_buckets() {
  let _buckets = Buckets::new(3, 5);
}

#[test]
fn fill_the_buckets() {
  let mut buckets = Buckets::new(3, 5);
  buckets = buckets.fill_left();
  assert_eq!((3, 0), buckets.get_contents());
  buckets = buckets.fill_right();
  assert_eq!((3, 5), buckets.get_contents());
}

#[test]
fn empty_filled_buckets() {
  let mut buckets = Buckets::new(3, 5);
  buckets = buckets.fill_left();
  buckets = buckets.fill_right();
  assert_eq!((3, 5), buckets.get_contents());
  buckets = buckets.empty_right();
  assert_eq!((3, 0), buckets.get_contents());
  buckets = buckets.empty_left();
  assert_eq!((0, 0), buckets.get_contents());
}

#[test]
fn transfer_smaller_bucket_contents() {
  let mut buckets = Buckets::new(3, 11);
  buckets = buckets.fill_left();
  assert_eq!((3, 0), buckets.get_contents());
  buckets = buckets.transfer_right();
  assert_eq!((0, 3), buckets.get_contents());
  buckets = buckets.fill_left();
  buckets = buckets.transfer_right();
  assert_eq!((0, 6), buckets.get_contents());
  buckets = buckets.fill_left();
  buckets = buckets.transfer_right();
  assert_eq!((0, 9), buckets.get_contents());
}

#[test]
fn transfer_larger_bucket_contents() {
  let mut buckets = Buckets::new(3, 11);
  buckets = buckets.fill_right();
  assert_eq!((0, 11), buckets.get_contents());
  buckets = buckets.transfer_left();
  assert_eq!((3, 8), buckets.get_contents());
  buckets = buckets.empty_left();
  assert_eq!((0, 8), buckets.get_contents());
  buckets = buckets.transfer_left();
  assert_eq!((3, 5), buckets.get_contents());
  buckets = buckets.empty_left();
  assert_eq!((0, 5), buckets.get_contents());
  buckets = buckets.transfer_left();
  assert_eq!((3, 2), buckets.get_contents());
  buckets = buckets.empty_left();
  assert_eq!((0, 2), buckets.get_contents());
  buckets = buckets.transfer_left();
  assert_eq!((2, 0), buckets.get_contents());
}

#[test]
fn transfer_into_partial_filled_bucket() {
  let mut buckets = Buckets::new(7, 11);
  buckets = buckets.fill_left();
  assert_eq!((7, 0), buckets.get_contents());
  buckets = buckets.transfer_right();
  assert_eq!((0, 7), buckets.get_contents());
  buckets = buckets.fill_left();
  assert_eq!((7, 7), buckets.get_contents());
  buckets = buckets.transfer_right();
  assert_eq!((3, 11), buckets.get_contents());
}
