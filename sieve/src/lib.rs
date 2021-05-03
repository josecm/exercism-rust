pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {

    let upper_bound = (upper_bound + 1) as usize;
    let mut sieve = vec![false; upper_bound];

    for i in 2..upper_bound {
        for j in (i..upper_bound).step_by(i).skip(1) {
            sieve[j] = true;
        }
    }

    (0..upper_bound as u64)
        .zip(sieve)
        .filter(|p| !p.1)
        .map(|p| p.0)
        .skip(2)
        .collect()
}
