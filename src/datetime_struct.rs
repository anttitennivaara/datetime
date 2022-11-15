use super::functions::*;

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

    pub fn add_months(&mut self, months: isize) {
        let mut months_left = months;
        loop {
            if self.month + months_left <= 12 {
                self.month += months_left;
                break;
            }
            months_left -= 12 - self.month + 1;
            self.month = 1;
            self.year += 1;
        }
    }

    pub fn add_days(&mut self, days: isize) {
        let mut days_left = days;
        loop {
            let mut month_length = MONTH_LENGTHS[self.month as usize];
            if is_leap_year(self.year) && self.month == 2 {
                month_length += 1;
            }
            if self.day + days_left <= month_length {
                self.day += days_left;
                break;
            }
            days_left -= month_length - self.day + 1;
            self.day = 1;
            self.add_months(1);
        }
    }

    pub fn add_hours(&mut self, hours: isize) {
        let mut hours_left = hours;
        loop {
            if self.hour + hours_left <= 23 {
                self.hour += hours_left;
                break;
            }
            hours_left -= 24 - self.hour;
            self.hour = 0;
            self.add_days(1);
        }
    }

    pub fn add_minutes(&mut self, minutes: isize) {
        let mut minutes_left = minutes;
        loop {
            if self.minute + minutes_left <= 59 {
                self.minute += minutes_left;
                break;
            }
            minutes_left -= 60 - self.minute;
            self.minute = 0;
            self.add_hours(1);
        }
    }

    pub fn add_seconds(&mut self, seconds: isize) {
        let mut seconds_left = seconds;
        loop {
            if self.second + seconds_left <= 59 {
                self.second += seconds_left;
                break;
            }
            seconds_left -= 60 - self.second;
            self.second = 0;
            self.add_minutes(1);
        }
    }
}