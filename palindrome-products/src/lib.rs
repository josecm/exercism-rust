#![feature(map_first_last)]

use std::collections::BTreeMap;

#[derive(Debug, PartialEq, Eq)]
pub struct Palindrome {
    factors: Vec<(u64, u64)>,
}

impl Palindrome {
    pub fn new(a: u64, b: u64) -> Palindrome {
        Self {
            factors: vec![(a, b)],
        }
    }

    pub fn value(&self) -> u64 {
        let (a, b) = self.factors[0];
        a * b
    }

    pub fn insert(&mut self, a: u64, b: u64) {
        self.factors.push((a, b));
    }
}

fn is_palindrome(n: u64) -> bool {
    let n = n.to_string();
    let rev = n.chars().rev().collect::<String>();
    n == rev
}

pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    let mut palindromes = BTreeMap::<u64, Palindrome>::new();
    for (a, b) in (min..=max)
        .flat_map(|a| (a..=max).map(move |b| (a, b)))
        .filter(|(a, b)| is_palindrome(a * b))
    {
        if let Some(p) = palindromes.get_mut(&(a * b)) {
            p.insert(a, b);
        } else {
            palindromes.insert(a * b, Palindrome::new(a, b));
        }
    }
    match (palindromes.pop_first(), palindromes.pop_last()) {
        (Some((_, x)), Some((_, y))) => Some((x, y)),
        _ => None,
    }
}
