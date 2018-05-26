#[derive(Debug)]
pub struct Luhn {
    text: String,
}

impl<T: ToString> From<T> for Luhn {
    fn from(input: T) -> Self {
        Luhn {
            text: input.to_string(),
        }
    }
}

impl Luhn {
    pub fn is_valid(&self) -> bool {
        let cleaned = self.text.replace(" ", "");
        let is_valid_sequence = |seq: &str| seq.chars().all(char::is_numeric);

        if cleaned.len() <= 1 || !is_valid_sequence(&cleaned) {
            false
        } else {
            Self::calculate_sum(&cleaned) % 10 == 0
        }
    }

    fn calculate_sum(sequence: &str) -> u32 {
        let is_odd = |x| x % 2 != 0;

        sequence
            .chars()
            .rev()
            .enumerate()
            .map(|(k, v)| {
                let mut value = v.to_digit(10).unwrap();

                if is_odd(k) {
                    value *= 2;
                    if value > 9 {
                        value -= 9;
                    }
                }
                value
            })
            .sum()
    }
}
