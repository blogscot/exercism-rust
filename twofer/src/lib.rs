pub fn twofer(name: &str) -> String {
    match name {
        "" => "One for you, one for me.".into(),
        _ => format!("One for {}, one for me.", name),
    }
}
