use std::collections::HashMap;

pub fn brackets_are_balanced(string: &str) -> bool {
    let map: HashMap<char, char> = vec![(')', '('), (']', '['), ('}', '{')]
        .into_iter()
        .collect();
    let mut stack = Vec::new();
    for c in string.chars() {
        if map.values().any(|&a| a == c) {
            stack.push(c);
        } else if let Some(a) = map.get(&c) {
            if Some(*a) != stack.pop() {
                return false
            }
        }
    }
    stack.is_empty()
}
