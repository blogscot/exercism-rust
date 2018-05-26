pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, String> {
    from(number, from_base).and_then(|num| to(num, to_base))
}

fn from(number: &[u32], base: u32) -> Result<u32, String> {
    let is_valid = |digits: &[u32], base| digits.iter().all(|x| x < base);

    if !is_valid(number, &base) || base < 2 {
        return Err("Invalid parameters!".into());
    }
    Ok(number.iter().fold(0, |acc, value| acc * base + value))
}

fn to(mut number: u32, base: u32) -> Result<Vec<u32>, String> {
    if base < 2 {
        return Err("Invalid parameters!".into());
    }
    let mut output = Vec::new();
    while number > 0 {
        let (quot, rem) = (number / base, number % base);
        output.push(rem);
        number = quot
    }
    Ok(output.into_iter().rev().collect())
}
