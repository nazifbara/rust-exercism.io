use chrono::{Duration, NaiveTime};
use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Clock {
    time: NaiveTime,
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.time.format("%H:%M"))
    }
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Clock {
            time: NaiveTime::from_hms(0, 0, 0)
                + Duration::hours(hours.into())
                + Duration::minutes(minutes.into()),
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock {
            time: self.time + Duration::minutes(minutes.into()),
        }
    }
}
