extern crate regex;

use regex::Regex;

pub fn encode(text: &str) -> String {
    let spl = split(text.to_string(), Vec::new()).1;

    let result: Vec<_> = spl.into_iter()
        .map(|x| (x.len(),x.chars().nth(0).unwrap()))
        .map(|(x,y)| transform(x,y)).collect();

    result.join("")
}

fn split(s: String, mut t: Vec<String>) -> (String, Vec<String>) {
    if s.is_empty() { return (s, t); }

    let first = first(&s);
    let iter = s.chars().into_iter();
    let head: Vec<_> = iter.clone().take_while(|&x| x==first).collect();
    let tail: Vec<_> = iter.skip_while(|&x| x==first).collect();

    let (x, y) = (concat(head), concat(tail));
    t.push(x);
    split(y, t)
}

fn transform(length: usize, ch: char) -> String {
    match length {
        1 => format!("{}", ch.to_string()),
        _ => format!("{}{}", length, ch.to_string()),
    }
}

fn first(s: &String) -> char {
    match s.chars().next() {
        None         => ' ',
        Some(result) => result,
    }
}

fn concat(chars: Vec<char>) -> String {
    chars.into_iter().map(|chr| chr.to_string()).collect::<Vec<_>>().join("")
}


pub fn decode(text: &str) -> String {
    let re = Regex::new(r"(\d{0,})(\D)").unwrap();

    let matches: Vec<_> = re.captures_iter(text).collect();
    let items: Vec<_> = matches.iter()
        .map(|x| &x[0])
        .map(|x| convert(x)).collect::<Vec<_>>();
    items.join("")
}

fn convert(s: &str) -> String {
    if s.len() == 1 {
        return s.into()
    } else {
        let (number, letter) = s.split_at(s.len()-1);
        let num: usize = number.parse().unwrap();
        letter.repeat(num)
    }
}
