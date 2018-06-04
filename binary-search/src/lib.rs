pub fn find<C, T>(input: C, value: T) -> Option<usize>
where
  C: AsRef<[T]>,
  T: Ord,
{
  let list = input.as_ref();
  let length = list.len();
  if length == 0 {
    return None;
  }
  let mut index = length / 2;
  let mut start = 0;
  let mut end = length;

  while list[index] != value {
    if value < list[index] {
      end = index;
    } else {
      start = index;
    }
    let new_index = (start + end) / 2;
    if new_index == index {
      return None;
    } else {
      index = new_index;
    }
  }
  Some(index)
}
