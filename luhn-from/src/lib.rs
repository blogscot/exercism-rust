#[derive(Debug)]
pub struct Luhn {
    text: String,
}

impl <T: ToString> From<T> for Luhn {
    fn from(input: T) -> Self {
        Luhn { text: input.to_string() }
    }
}

impl Luhn {
    pub fn is_valid(&self) -> bool {
        let cleaned = self.text.replace(" ", "");
        if cleaned.len() <= 1 || !Self::is_valid_sequence(&cleaned) {
            return false;
        }
        Self::calculate_sum(&cleaned) % 10 == 0
    }

    fn calculate_sum(sequence: &str) -> u32 {
        sequence
            .chars()
            .rev()
            .enumerate()
            .map(|(k, v)| {
                let mut value = v.to_digit(10).unwrap();

                if is_odd(k) {
                    value = value * 2;
                    if value > 9 {
                        value -= 9;
                    }
                }
                value
            })
            .sum::<u32>()
    }

    fn is_valid_sequence(sequence: &str) -> bool {
        sequence.chars().all(char::is_numeric)
    }
}

fn is_odd(x: usize) -> bool {
    x % 2 != 0
}
