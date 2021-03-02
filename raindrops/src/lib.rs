pub fn raindrops(n: u32) -> String {
    let mut res = String::new();
    let map = [(3, "Pling"), (5, "Plang"), (7, "Plong")];

    for (f, s) in map.iter() {
        if n % f == 0 {
            res.push_str(s)
        }
    }

    if res.is_empty() {
        n.to_string()
    } else {
        res
    }
}
