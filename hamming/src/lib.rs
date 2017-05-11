
pub fn hamming_distance(strand1: &str, strand2: &str) -> Result<usize, String> {
    if strand1.len() != strand2.len() {
        return Err("Unequal strand lengths!".into());
    }

    Ok(strand1
           .chars()
           .zip(strand2.chars())
           .filter(|&(x, y)| x != y)
           .count())
}
