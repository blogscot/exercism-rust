pub fn difference(number: usize) -> usize {
    square_of_sum(number) - sum_of_squares(number)
}

pub fn square_of_sum(number: usize) -> usize {
    let value: usize = (1..=number).sum();
    value * value
}

pub fn sum_of_squares(number: usize) -> usize {
    (1..=number).map(|x| x * x).sum()
}
