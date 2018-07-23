const DISCOUNT: [usize; 6] = [0, 0, 5, 10, 20, 25];
const BOOK_PRICE: usize = 800;

fn contains_duplicates(list: &[usize]) -> bool {
  let mut list = list.to_vec();
  while let Some(item) = list.pop() {
    if list.contains(&item) {
      return true;
    }
  }
  false
}

fn find_duplicates(list: &[usize]) -> (Vec<usize>, Vec<usize>) {
  let mut list = list.to_vec();
  let mut duplicates = vec![];
  let mut original = vec![];
  while let Some(item) = list.pop() {
    if list.contains(&item) {
      duplicates.push(item);
    } else {
      original.push(item)
    }
  }
  duplicates.reverse();
  original.reverse();
  (original, duplicates)
}

pub fn group(list: &[usize]) -> Vec<Vec<usize>> {
  let mut duplicates = find_duplicates(&list);
  if contains_duplicates(&duplicates.1) {
    let mut result = vec![duplicates.0];
    while contains_duplicates(&duplicates.1) {
      duplicates = find_duplicates(&duplicates.1);
      result.push(duplicates.0);
    }
    result.push(duplicates.1);
    result
  } else {
    vec![duplicates.0, duplicates.1]
  }
}

fn redistribute(group: &mut [usize]) {
  while group.contains(&5) && group.contains(&3) {
    let five_pos = group.iter().position(|&num| num == 5).unwrap();
    let three_pos = group.iter().position(|&num| num == 3).unwrap();
    group[five_pos] = 4;
    group[three_pos] = 4;
  }
}

fn get_price(num_books: usize) -> usize {
  let discount_rate = 100 - DISCOUNT[num_books];
  num_books * BOOK_PRICE * discount_rate / 100
}

pub fn lowest_price(list: &[usize]) -> usize {
  let groups = group(list);
  let mut books: Vec<usize> = groups.iter().map(|group| group.len()).collect();
  redistribute(&mut books);
  books
    .into_iter()
    .fold(0usize, |acc, group| acc + get_price(group))
}
