use std::collections::HashMap;

pub fn count(nucleo: char, sequence: &str) -> Result<usize, &str> {
    if !is_valid(nucleo) || !is_valid_sequence(sequence) {
        return Err("Invalid Symbol found!");
    }
    Ok(sequence.chars().filter(|&s| s == nucleo).count())
}

pub fn nucleotide_counts(sequence: &str) -> Result<HashMap<char, usize>, &str> {
    let mut letters = HashMap::new();
    letters.insert('A', count('A', sequence)?);
    letters.insert('C', count('C', sequence)?);
    letters.insert('G', count('G', sequence)?);
    letters.insert('T', count('T', sequence)?);
    Ok(letters)
}

fn is_valid_sequence(sequence: &str) -> bool {
    sequence.chars().all(is_valid)
}

fn is_valid(nucleo: char) -> bool {
    "ACGT".contains(nucleo)
}
