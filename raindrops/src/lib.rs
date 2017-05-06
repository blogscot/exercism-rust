pub fn raindrops(number: u32) -> String {
    let mut s = String::new();

    if number % 3 == 0 {
        s += "Pling";
    }
    if number % 5 == 0 {
        s += "Plang";
    }
    if number % 7 == 0 {
        s += "Plong";
    }

    match s.as_ref() {
        "" => number.to_string(),
        _  => s,
    }
}
