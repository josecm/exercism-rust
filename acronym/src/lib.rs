// fn capitalize(s: &str) -> String {
//     let mut s = String::from(s);
//     if let c = s.get_mut(0usize) {
//         c = c.to_ascii_uppercase();
//     }
//     s
// }

pub fn abbreviate(phrase: &str) -> String {
    phrase
        .split(|c: char| c.is_whitespace() || (c.is_ascii_punctuation() && c != '\''))
        .flat_map(|s| {
            s.chars().take(1).chain(
                s.chars()
                    .skip_while(char::is_ascii_uppercase)
                    .filter(char::is_ascii_uppercase),
            )
        })
        .collect::<String>()
        .to_uppercase()
}
