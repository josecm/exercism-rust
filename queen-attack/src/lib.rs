#[derive(Debug)] 
pub struct ChessPosition(i32, i32);

#[derive(Debug)]
pub struct Queen(ChessPosition);

impl ChessPosition {
    pub fn new(rank: i32, file: i32) -> Option<Self> {
        if [rank, file].iter().all(|c| (0..8).contains(c)) {
            Some(ChessPosition(rank, file))
        } else {
            None
        }
    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Queen(position)
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        let xd = i32::abs(self.0.0 - other.0.0);
        let yd = i32::abs(self.0.1 - other.0.1);
        xd == 0 || yd == 0 || xd == yd
    }
}
