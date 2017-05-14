pub fn lsp(sequence: &str, size: usize) -> Result<u32, &str> {
    if sequence.len() < size || !sequence.chars().all(char::is_numeric) {
        return Err("Invalid parameters.");
    } else if size == 0 {
        return Ok(1);
    }

    let digits: Vec<u32> = sequence
        .chars()
        .map(|ch| ch.to_digit(10).unwrap())
        .collect();

    Ok(digits
           .windows(size)
           .map(|window| window.iter().product())
           .max()
           .unwrap())
}
