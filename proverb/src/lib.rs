pub fn build_proverb(list: Vec<&str>) -> String {
    let mut output: String = String::new();
    if list.is_empty() {
        return output;
    }
    for pair in list.windows(2) {
        if let [first, second] = pair {
            output += &format!("For want of a {} the {} was lost.\n", first, second);
        }
    }
    output += &format!("And all for the want of a {}.", list[0]);
    output
}
