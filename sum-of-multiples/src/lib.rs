
pub fn sum_of_multiples(limit : usize, factors: &Vec<usize>) -> usize {
    let mut result = 0;

    for x in 1..limit {
        for y in factors {
            if x % y == 0 {
                result += x;
                break;
            }
        }
    }
    result
}
