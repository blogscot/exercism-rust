extern crate regex;
use regex::Regex;

pub fn abbreviate(text: &str) -> String {
    let re = Regex::new(r"\b\w|[A-Z][a-z]").unwrap();
    let mut result: Vec<String> = Vec::new();
    for caps in re.captures_iter(text) {
        result.push(caps[0][..1].into());
    };
    result.join("").to_uppercase()
}
