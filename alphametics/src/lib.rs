use std::collections::HashMap;
use std::collections::HashSet;
use std::rc::Rc;

struct Permutations {
    chars: String,
    count: usize,
}

impl Permutations {
    fn new(input: &str) -> Self {
        let chars = ('A'..='Z')
            .filter(|c| input.contains(&c.to_string()))
            .collect::<String>();
        let count = 10usize.pow(chars.len() as u32 - 1);
        Permutations { chars, count }
    }

    fn unique_digits(d: usize) -> bool {
        let d = d.to_string();
        (0..=9)
            .map(|i| d.chars().filter(|&c| c == char::from_digit(i,10).unwrap()).count())
            .all(|n| n <= 1)
    }


}

impl Iterator for Permutations {
    type Item = HashMap<char, u8>;

    fn next(&mut self) -> Option<Self::Item> {
        while !Self::unique_digits(self.count) && self.count < 10usize.pow(self.chars.len() as u32) {
            self.count += 1;
        }
        if self.count < 10usize.pow(self.chars.len() as u32) {
            self.count += 1;
            Some(self.chars.chars().zip(self.count.to_string().chars().map(|n| n.to_digit(10).unwrap() as u8)).collect())
        } else {
            None
        }
    }
}

fn parse(input: &str) -> (Vec<&str>, &str) {
    if let Some((lhs, rhs)) = input.split_once("==") {
        (lhs.split("+").map(|s| s.trim()).collect(), rhs.trim())
    } else {
        panic!()
    }
}

fn convert(s: &str, sol: &HashMap<char, u8>) -> u32 {
    s.chars()
        .map(|c| char::from_digit(*sol.get(&c).unwrap() as u32, 10).unwrap())
        .collect::<String>()
        .parse::<u32>()
        .unwrap()
}

fn check(input: &(Vec<&str>, &str), sol: &HashMap<char, u8>) -> bool {
    let sum: u32 = input.0.iter().map(|s| convert(s, &sol)).sum();
    let res = convert(input.1, &sol);
    sum == res
}

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let parsed_input = parse(input);
    let permutations = Permutations::new(input);
    for sol in permutations {
        if check(&parsed_input, &sol) {
            return Some(sol);
        }
    }
    None
}
