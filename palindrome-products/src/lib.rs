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
        assert!(self.factors.len() > 0);
        self.factors[0].0 * self.factors[0].1
    }

    pub fn insert(&mut self, a: u64, b: u64) {
        self.factors.push((a, b));
    }
}

fn is_palindrome(n: u64) -> bool {
    let mut tmp = n;
    let mut m = 0;
    while tmp > 0 {
        m = m * 10 + tmp % 10;
        tmp = tmp / 10;
    }
    n == m
}

pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    let mut products = (min..=max)
        .flat_map(|a| (a..=max).map(move |b| (a, b)))
        .filter(|(a, b)| is_palindrome(a * b))
        .collect::<Vec<_>>();
    products.sort_by_key(|(a, b)| a * b);
    let min = products.first()?;
    let max = products.last()?;
    let min_factors = products
        .iter()
        .copied()
        .take_while(|(a, b)| a * b == min.0 * min.1)
        .collect::<Vec<_>>();
    let max_factors = products
        .iter()
        .copied()
        .skip_while(|(a, b)| a * b != max.0 * max.1)
        .collect::<Vec<_>>();
    Some((
        Palindrome {
            factors: min_factors,
        },
        Palindrome {
            factors: max_factors,
        },
    ))
}
