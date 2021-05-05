// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(PartialEq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

pub struct Robot {
    coord: (i32, i32),
    dir: Direction,
}

impl Robot {
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        Robot { coord: (x, y), dir: d }
    }

    pub fn turn_right(self) -> Self {
        Robot {
            dir: match self.dir {
               Direction::North => Direction::East,
               Direction::East  => Direction::South,
               Direction::South => Direction::West,
               Direction::West  => Direction::North,
            },
            ..self 
        }
    }

    pub fn turn_left(self) -> Self {
        Robot {
            dir: match self.dir {
               Direction::North => Direction::West,
               Direction::East  => Direction::North,
               Direction::South => Direction::East,
               Direction::West  => Direction::South,
            },
            ..self 
        }
 
    }

    pub fn advance(self) -> Self {
        let (x, y) = self.coord;
        Robot {
            coord: match self.dir {
               Direction::North => (x, y+1),
               Direction::East  => (x+1, y),
               Direction::South => (x, y-1),
               Direction::West  => (x-1, y),
            },
            ..self
        }
    }

    pub fn instructions(self, instructions: &str) -> Self {
        instructions.chars().fold(self, |robot, i| match i {
                'A' => robot.advance(),
                'L' => robot.turn_left(),
                'R' => robot.turn_right(),
                _ => panic!("unknown instruction \'{}\'", i),
        })
    }

    pub fn position(&self) -> (i32, i32) {
        self.coord
    }

    pub fn direction(&self) -> &Direction {
        &self.dir
    }
}
