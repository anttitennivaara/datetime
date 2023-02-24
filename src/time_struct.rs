use crate::functions::*;

pub struct Time {
    pub nanoseconds: isize
}

impl Time {
    pub fn to_unit(&self, unit: &str) -> isize {
        match unit {
            "hours" => seconds_to_hours(nanoseconds_to_seconds(self.nanoseconds)),
            "minutes" => seconds_to_minutes(nanoseconds_to_seconds(self.nanoseconds)),
            "seconds" => nanoseconds_to_seconds(self.nanoseconds),
            "microseconds" => nanoseconds_to_microseconds(self.nanoseconds),
            "nanoseconds" => self.nanoseconds,
            _ => panic!("Invalid unit"),
        }
    }

    pub fn add_time(&mut self, time: isize, unit: &str) {
        self.nanoseconds += match unit {
            "nanoseconds" => time,
            "microseconds" => microseconds_to_nanoseconds(time),
            "seconds" => seconds_to_nanoseconds(time),
            "minutes" => minutes_to_nanoseconds(time),
            "hours" => hours_to_nanoseconds(time),
            _ => panic!("Invalid unit"),
        }
    }
}

pub fn build_time(time: isize, unit: &str) -> Time {
    Time {
        nanoseconds: match unit {
            "nanoseconds" => time,
            "microseconds" => microseconds_to_nanoseconds(time),
            "seconds" => seconds_to_nanoseconds(time),
            "minutes" => minutes_to_nanoseconds(time),
            "hours" => hours_to_nanoseconds(time),
            _ => panic!("Invalid unit"),
        }
    }
}