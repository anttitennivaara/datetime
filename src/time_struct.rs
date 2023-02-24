use crate::functions::*;

pub struct Time {
    nanoseconds: isize
}

impl Time {
    pub fn to_unit(&self, unit: &str) -> isize {
        match unit {
            "hours" => seconds_to_hours(nanoseconds_to_seconds(self.nanoseconds)),
            "minutes" => seconds_to_minutes(nanoseconds_to_seconds(self.nanoseconds)),
            "seconds" => nanoseconds_to_seconds(self.nanoseconds),
            "microseconds" => seconds_to_hours(nanoseconds_to_seconds(self.nanoseconds)),
            "nanoseconds" => self.nanoseconds,
            _ => panic!("Invalid unit"),
        }
    }

    pub fn add_time(&mut self, time: isize, unit: &str) {
        self.nanoseconds += match unit {
            "nanoseconds" => time,
            "microseconds" => time,
            "seconds" => time,
            "minutes" => time,
            "hours" => time,
            _ => panic!("Invalid unit"),
        }
    }
}