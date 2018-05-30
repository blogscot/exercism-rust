pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
  fn get_row(n: usize, input: &[Vec<u64>]) -> Vec<u64> {
    input[n].clone()
  }
  fn get_column(n: usize, input: &[Vec<u64>]) -> Vec<u64> {
    input.iter().map(|row| row[n]).collect()
  }
  fn is_greater_or_equal(n: u64, vector: &[u64]) -> bool {
    vector.iter().all(|&value| value <= n)
  }
  fn is_smaller_or_equal(n: u64, vector: &[u64]) -> bool {
    vector.iter().all(|&value| n <= value)
  }
  let mut output = vec![];
  for row_index in 0..input.len() {
    let row = get_row(row_index, input);
    for col_index in 0..input[0].len() {
      let column = get_column(col_index, input);
      let value = row[col_index];
      if is_greater_or_equal(value, &row) && is_smaller_or_equal(value, &column) {
        output.push((row_index, col_index));
      }
    }
  }
  output
}
