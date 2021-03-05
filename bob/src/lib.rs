pub fn reply(message: &str) -> &str {
    let message = message.trim();
    let all_capitals = message.chars().any(|c| c.is_alphabetic())
        && message.chars().all(|c| !c.is_alphabetic() || c.is_uppercase());
    let is_question = message.ends_with('?');
    let is_nothing = message.is_empty();

    match (all_capitals, is_question, is_nothing) {
        (true, true, _) => "Calm down, I know what I'm doing!",
        (true, _, _) => "Whoa, chill out!",
        (_, true, _) => "Sure.",
        (_, _, true) => "Fine. Be that way!",
        _ => "Whatever.",
    }
}
