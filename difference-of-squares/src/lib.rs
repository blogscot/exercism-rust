
pub fn difference(number: usize) -> usize {
    square_of_sum(number) - sum_of_squares(number)
}

pub fn square_of_sum(number : usize) -> usize {
    (1..number+1)
        .fold(0, |sum, x| sum + x)
        .pow(2)
}

pub fn sum_of_squares(number : usize) -> usize {
    (1..number+1)
        .map(|x| x * x)
        .fold(0, |sum, x| sum + x)
}
