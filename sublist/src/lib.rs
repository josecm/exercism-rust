#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

fn is_sublist<T: PartialEq>(f: &[T], s: &[T]) -> bool {
    if s.len() <= f.len() { return false }
    s.windows(f.len()).any(|w| w == f)
}

pub fn sublist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> Comparison {
    match (first_list, second_list) {
        (f, s) if s == f => Comparison::Equal,
        (f, s) if is_sublist(f, s) => Comparison::Sublist,
        (f, s) if is_sublist(s, f) => Comparison::Superlist,
        _ => Comparison::Unequal,
    }
}
