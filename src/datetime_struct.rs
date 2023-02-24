use std::ops::Sub;
use num::signum;

use crate::functions::*;

pub struct DateTime {
    pub year: isize,
    pub month: isize,
    pub day: isize,
    pub hour: isize,
    pub minute: isize,
    pub second: isize,
}

impl Default for DateTime {
    fn default() -> Self {
        Self::new()
    }
}

impl Sub for DateTime {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        let mut result = Self {
            year: 0,
            month: 0,
            day: 0,
            hour: 0,
            minute: 0,
            second: 0,
        };
        result.add_years(self.year - other.year);
        result.add_months(self.month - other.month);
        result.add_days(self.day - other.day);
        result.add_hours(self.hour - other.hour);
        result.add_minutes(self.minute - other.minute);
        result.add_seconds(self.second - other.second);
        result
    }
}

impl DateTime {
    pub fn new() -> Self {
        Self {
            year: 1970,
            month: 1,
            day: 1,
            hour: 0,
            minute: 0,
            second: 0,
        }
    }
    
    pub fn to_unix_time(&self) -> isize {
        let mut seconds = self.second;
        seconds += minutes_to_seconds(self.minute);
        seconds += hours_to_seconds(self.hour);
        let mut days = self.day - 1;
        days += months_to_days(1, self.month, self.year);
        days += years_to_days(1970, self.year);
        seconds += days_to_seconds(days);
        seconds
    }
    
    pub fn from_unix_time(seconds: isize) -> DateTime {
        let mut datetime = DateTime::new();
        datetime.add_seconds(seconds);
        datetime
    }

    pub fn to_string(&self) -> String {
        let mut date_string = String::from("");
        date_string.push_str(self.year.to_string().as_str());
        date_string.push_str(".");
        date_string.push_str(leading_zero_string(self.month, 2).as_str());
        date_string.push_str(".");
        date_string.push_str(leading_zero_string(self.day, 2).as_str());
        date_string.push_str(" ");
        date_string.push_str(leading_zero_string(self.hour, 2).as_str());
        date_string.push_str(":");
        date_string.push_str(leading_zero_string(self.minute, 2).as_str());
        date_string.push_str(":");
        date_string.push_str(leading_zero_string(self.second, 2).as_str());
        date_string
    }

    pub fn to_units_string(&self) -> String {
        let mut date_string = String::from("");
        date_string.push_str("Years: ");
        date_string.push_str(self.year.to_string().as_str());
        date_string.push_str("\nMonths: ");
        date_string.push_str(self.month.to_string().as_str());
        date_string.push_str("\nDays: ");
        date_string.push_str(self.day.to_string().as_str());
        date_string.push_str("\nHours: ");
        date_string.push_str(self.hour.to_string().as_str());
        date_string.push_str("\nMinutes: ");
        date_string.push_str(self.minute.to_string().as_str());
        date_string.push_str("\nSeconds: ");
        date_string.push_str(self.second.to_string().as_str());
        date_string
    }

    pub fn time_since(&self, other: DateTime, unit: &str) -> isize {
        let self_unix = self.to_unix_time();
        let other_unix = other.to_unix_time();
        let unix_time_diff = self_unix - other_unix;
        match unit {
            "seconds" => return unix_time_diff,
            "minutes" => return seconds_to_minutes(unix_time_diff),
            "hours" => return seconds_to_hours(unix_time_diff),
            "days" => return seconds_to_days(unix_time_diff),
            _ => panic!("Invalid unit"),
        }
    }
    
    pub fn add_time(&mut self, time: isize, unit: &str) {
        match unit {
            "years" => self.add_years(time),
            "months" => self.add_months(time),
            "days" => self.add_days(time),
            "hours" => self.add_hours(time),
            "minutes" => self.add_minutes(time),
            "seconds" => self.add_seconds(time),
            _ => panic!("Invalid unit"),
        }
    }

    pub fn add_years(&mut self, years: isize) {
        self.year += years;
    }

    pub fn add_months(&mut self, months: isize) {
        self.month += months % 12;
        self.year += months / 12;
        if !is_between(1, 12, self.month) {
            self.add_years(signum(self.month));
            self.month -= 12 * signum(self.month);
        }
    }

    pub fn add_days(&mut self, days: isize) {
        let mut days_left = days;
        loop {
            if days_left == 0 {
                break;
            }
            let month_length = get_month_length(self.month, self.year);
            let sum = self.day + days_left;
            if is_between(1, month_length, sum) {
                self.day += days_left;
                break;
            }
            if sum > month_length {
                days_left -= month_length - self.day + 1;
                self.add_months(1);
                self.day = 1;
                continue;
            }
            if sum <= 0 {
                days_left -= self.day;
                self.add_months(-1);
                self.day = get_month_length(self.month, self.year);
            }
        }
    }

    pub fn add_hours(&mut self, hours: isize) {
        self.hour += hours % 24;
        self.add_days(hours / 24);
        if !is_between(0, 23, self.hour) {
            self.add_days(signum(self.hour));
            self.hour -= 24 * signum(self.hour);
        }
    }

    pub fn add_minutes(&mut self, minutes: isize) {
        self.minute += minutes % 24;
        self.add_hours(minutes / 24);
        if !is_between(0, 23, self.minute) {
            self.add_hours(signum(self.minute));
            self.minute -= 24 * signum(self.minute);
        }
    }

    pub fn add_seconds(&mut self, seconds: isize) {
        self.second += seconds % 24;
        self.add_minutes(seconds / 24);
        if !is_between(0, 23, self.second) {
            self.add_minutes(signum(self.second));
            self.second -= 24 * signum(self.second);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::datetime_struct::*;

    #[test]
    fn test_add_time() {
        let mut test_datetime = DateTime::new();
        test_datetime.add_time(13, "months");
        assert_eq!(test_datetime.year, 1971);
    }
}