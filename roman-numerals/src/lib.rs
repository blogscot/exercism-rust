pub struct Roman {
    symbol: &'static str,
    value: u32,
}

const NUMERALS: [Roman; 13] = [
    Roman { symbol: "I",  value: 1 },
    Roman { symbol: "IV", value: 4 },
    Roman { symbol: "V",  value: 5 },
    Roman { symbol: "IX", value: 9 },
    Roman { symbol: "X",  value: 10 },
    Roman { symbol: "XL", value: 40 },
    Roman { symbol: "L",  value: 50 },
    Roman { symbol: "XC", value: 90 },
    Roman { symbol: "C",  value: 100 },
    Roman { symbol: "CD", value: 400 },
    Roman { symbol: "D",  value: 500 },
    Roman { symbol: "CM", value: 900 },
    Roman { symbol: "M",  value: 1000 },
];

impl Roman {
    pub fn from(mut number: u32) -> String {
        let mut roman: String = String::new();
        for numeral in NUMERALS.iter().rev() {
            while number >= numeral.value {
                roman += numeral.symbol;
                number -= numeral.value;
            }
        }
        roman
    }
}
