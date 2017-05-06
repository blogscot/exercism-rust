
pub fn verse(n : i32) -> String {
    format!("{} of beer on the wall, {} of beer.
{}, {} of beer on the wall.
", pluralise(n, true), pluralise(n, false), action(n), pluralise(n - 1, false))
}

pub fn sing(start : i32, end : i32) -> String {
    let mut result : Vec<String> = Vec::new();
    for n in (end..start+1).rev() {
        result.push(verse(n));
    }
    result.join("\n")
}

fn pluralise(number: i32, capitalise : bool) -> String {
    match number {
        number if number == 1               => "1 bottle".into(),
        number if number == 0 && capitalise => "No more bottles".into(),
        number if number == 0               => "no more bottles".into(),
        number if number == -1              => "99 bottles".into(),
        _                                   => format!("{} bottles", number),
    }
}

fn action<'a>(number: i32) -> &'a str {
    match number {
        number if number == 1 => "Take it down and pass it around",
        number if number == 0 => "Go to the store and buy some more",
        _                     => "Take one down and pass it around"
    }
}
