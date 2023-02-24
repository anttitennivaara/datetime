use crate::functions::*;

#[derive(Copy, Clone)]
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

    pub fn to_string(&self) -> String {
        let hours = self.to_unit("hours");
        let minutes = self.to_unit("minutes") - hours_to_minutes(hours);
        let seconds = self.to_unit("seconds") - minutes_to_seconds(self.to_unit("minutes"));
        let mut time_string = String::from("");
        time_string.push_str(leading_zero_string(hours, 2).as_str());
        time_string.push_str(":");
        time_string.push_str(leading_zero_string(minutes, 2).as_str());
        time_string.push_str(":");
        time_string.push_str(leading_zero_string(seconds, 2).as_str());
        time_string.push_str(".");
        time_string.push_str(leading_zero_string(self.nanoseconds, 6).as_str());
        time_string
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