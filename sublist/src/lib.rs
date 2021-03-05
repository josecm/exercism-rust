#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

fn is_sublist<T: PartialEq>(f: &[T], s: &[T]) -> bool {
    if s.len() <= f.len() { return false }
    (0..=s.len()-f.len())
        .map(|n| s.iter().skip(n))
        .any(|l| l.zip(f).all(|(a, b)| a == b))
}

pub fn sublist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> Comparison {
    match (first_list, second_list) {
        (f, s) if s == f => Comparison::Equal,
        (f, s) if is_sublist(f, s) => Comparison::Sublist,
        (f, s) if is_sublist(s, f) => Comparison::Superlist,
        _ => Comparison::Unequal,
    }
}
