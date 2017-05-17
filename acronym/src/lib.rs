extern crate regex;
use regex::Regex;

pub fn abbreviate(text: &str) -> String {
    let re = Regex::new(r"[A-Z][a-z]|\b[A-Z]|\b\w").unwrap();
    let mut result: Vec<String> = Vec::new();
    for caps in re.captures_iter(text) {
        save(&mut result, &caps[0][..1]);
    };
    result.join("").to_uppercase()
}

fn save(store: &mut Vec<String>, value: &str) {
    store.push(value.into());
}
