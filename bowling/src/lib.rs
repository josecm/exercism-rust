#[derive(Debug, PartialEq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

#[derive(Default, Debug)]
pub struct BowlingGame {
    frame: u16,
    frame_first_throw: bool,
    pins_up: u16,
    points: u16,
    bonus: (u16, u16),
    fill_balls: u16,
}

impl BowlingGame {
    pub fn new() -> Self {
        Self {
            pins_up: 10,
            frame_first_throw: true,
            ..Default::default()
        }
    }

    fn finished(&self) -> bool {
        self.frame >= 10 && self.fill_balls == 0
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {

        if self.finished() {
            return Err(Error::GameComplete);
        } else if self.pins_up < pins {
            return Err(Error::NotEnoughPinsLeft);
        }

        self.pins_up -= pins;
        self.points += pins * (1 + self.bonus.0);
        if self.fill_balls > 0 { self.fill_balls -= 1; }
        self.bonus = (self.bonus.1, 0);

        if self.frame_first_throw {
            if self.pins_up == 0 {
                self.pins_up = 10;
                if self.frame == 9 {
                    self.fill_balls = 2;
                } else if self.frame < 9 {
                    self.bonus = (self.bonus.0 + 1, 1);
                }
                self.frame += 1;
            } else {
                if self.frame == 10 { self.frame += 1; }
                self.frame_first_throw = false;
            }
        } else {
            if self.pins_up == 0 {
                if self.frame == 9 {
                    self.fill_balls = 1;
                } else if self.frame < 9 {
                   self.bonus = (self.bonus.0 + 1, 0); 
                }
            }
            self.pins_up = 10;
            self.frame += 1;
            self.frame_first_throw = true;
        }

        Ok(())
    }

    pub fn score(&self) -> Option<u16> {
        if !self.finished() {
            return None;
        }

        Some(self.points)
    }
}
