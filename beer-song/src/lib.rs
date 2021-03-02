pub fn verse(n: u32) -> String {
    let bottle = |n| if n > 1 { "bottles" } else { "bottle" };
    match n {
        0 => format!("No more bottles of beer on the wall, no more bottles of beer.\n\
                     Go to the store and buy some more, 99 bottles of beer on the wall.\n"),
        1 => format!("1 bottle of beer on the wall, 1 bottle of beer.\n\
                      Take it down and pass it around, no more bottles of beer on the wall.\n"),
        _ => format!("{} {} of beer on the wall, {} {} of beer.\n\
                      Take one down and pass it around, {} {} of beer on the wall.\n",
                      n, bottle(n), n, bottle(n), n-1, bottle(n-1))
    }
}

pub fn sing(start: u32, end: u32) -> String {
    (end..=start).rev().map(verse).collect::<Vec<String>>().join("\n")
}
