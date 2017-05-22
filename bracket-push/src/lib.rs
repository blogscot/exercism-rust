pub struct Brackets {
    text: String,
}

impl Brackets {
    pub fn from(brackets: &str) -> Self {
        Brackets { text: brackets.into() }
    }

    pub fn are_balanced(&self) -> bool {
        let mut buffer = String::from("");

        for ch in self.text.chars() {
            match ch {
                '(' => buffer.push(')'),
                '{' => buffer.push('}'),
                '[' => buffer.push(']'),
                ')' | '}' | ']' => {
                    if buffer.pop() != Some(ch) {
                        return false;
                    }
                }
                _ => (),
            }
        }
        buffer.is_empty()
    }
}
