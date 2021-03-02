pub fn factors(n: u64) -> Vec<u64> {

    let mut r = Vec::new();
    let mut n = n;

    while n != 1 {
        if let Some(f) = (2..=n).find(|f| n % f == 0) {
            r.push(f);
            n = n / f;
        }
    }
    
    return r
}
