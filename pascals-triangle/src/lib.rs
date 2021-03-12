pub struct PascalsTriangle(u32);

impl PascalsTriangle {
    fn pascal(c: u32, r: u32) -> u32 {
        match (c, r) {
            (0, _) => 1,
            _ if c > r => 0,
            _ => Self::pascal(c, r - 1) + Self::pascal(c - 1, r - 1),
        }
    }

    pub fn new(row_count: u32) -> Self {
        PascalsTriangle(row_count)
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        (0..self.0)
            .map(|r| (0..=r).map(|c| Self::pascal(c, r)).collect())
            .collect()
    }
}
