use std::fmt;

const MINUTES_IN_HOUR: i32 = 60;
const HOURS_IN_DAY: i32 = 24;

#[derive(Debug, PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Clock {
            hours: (hours + minutes.div_euclid(MINUTES_IN_HOUR)).rem_euclid(HOURS_IN_DAY),
            minutes: minutes.rem_euclid(MINUTES_IN_HOUR),
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock::new(self.hours, self.minutes + minutes)
    }
}
