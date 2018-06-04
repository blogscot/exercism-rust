pub fn map_function<F: Fn(i32) -> i32>(input: Vec<i32>, f: F) -> Vec<i32> {
  let mut output = vec![];
  for value in input {
    output.push(f(value))
  }
  output
}

pub fn map_closure<F: Fn(i32) -> i32>(input: Vec<i32>, f: F) -> Vec<i32> {
  map_function(input, f)
}
