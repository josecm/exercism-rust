use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

const MIN_PER_HOUR: i32 = 60;
const MIN_PER_DAY: i32 = MIN_PER_HOUR * 24;

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let total_minutes = 
            ((hours * MIN_PER_HOUR + minutes) % MIN_DAY + MIN_DAY) % MIN_DAY;
        Clock {
            hours: total_minutes / 60,
            minutes: total_minutes % 60,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Self::new(self.hours, self.minutes + minutes)
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}
