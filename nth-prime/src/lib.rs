
pub fn nth(n: u32) -> u32 {
    let is_prime = |n| (2..n).all(|i| n % i != 0);
    (2..).filter(|&i| is_prime(i)).nth(n as usize).unwrap()
}
