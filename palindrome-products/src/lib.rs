#![feature(map_first_last)]

use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq)]
pub struct Palindrome {
    value: u64,
    factors: HashSet<(u64, u64)>,
}

impl Palindrome {
    pub fn new(a: u64, b: u64) -> Palindrome {
        Self {
            value: a * b,
            factors: vec![(a,b)].into_iter().collect::<HashSet<_>>(),
        }
    }

    pub fn value(&self) -> u64 {
        self.value
    }

    pub fn insert(&mut self, a: u64, b: u64) {
        self.factors.insert((a, b));
    }
}

fn is_palindrome(n: &u64) -> bool {
    let n = n.to_string();
    let rev = n.chars().rev().collect::<String>();
    n == rev
}

pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    let mut products = (min..=max)
        .flat_map(|a| (a..=max).map(move |b| (a*b, (a, b)))).collect::<Vec<_>>();
    products.sort_by_key(|(n, _)| *n);
    let min_palindrome = products.iter().filter(|(n,_)| is_palindrome(n)).next()?;
    let max_palindrome = products.iter().rev().filter(|(n,_)| is_palindrome(n)).next()?; 
    let mut min_palindrome = Palindrome::new(min_palindrome.1.0, min_palindrome.1.1); 
    let mut max_palindrome = Palindrome::new(max_palindrome.1.0, max_palindrome.1.1); 
    for (n, (a,b)) in products {
        if n == min_palindrome.value() {
            min_palindrome.insert(a, b);
        } else if n == max_palindrome.value() {
            max_palindrome.insert(a, b);
        }
    }
    Some((min_palindrome, max_palindrome))
}
