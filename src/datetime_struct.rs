use super::functions::*;

pub struct DateTime {
    pub year: usize,
    pub month: usize,
    pub day: usize,
    pub hour: usize,
    pub minute: usize,
    pub second: usize,
}

impl DateTime {
    pub fn to_unix_time(&self) -> usize {
        let mut seconds = self.second;
        seconds += minutes_to_seconds(self.minute);
        seconds += hours_to_seconds(self.hour);
        let mut days = self.day - 1;
        days += months_to_days(1, self.month, self.year);
        days += years_to_days(1970, self.year);
        seconds += days_to_seconds(days);
        seconds
    }

    pub fn add_seconds_stupid(&mut self, seconds: usize) {
        for _ in 0..seconds {
            self.second += 1;
            if self.second < 60 {
                continue;
            }
            self.second = 0;
            self.minute += 1;
            if self.minute < 60 {
                continue;
            }
            self.minute = 0;
            self.hour += 1;
            if self.hour < 24 {
                continue;
            }
            self.hour = 0;
            self.day += 1;
            let mut month_length = MONTH_LENGTHS[self.month];
            if is_leap_year(self.year) && self.month == 2 {
                month_length += 1;
            }
            if self.day <= month_length {
                continue;
            }
            self.day = 1;
            self.month += 1;
            if self.month <= 12 {
                continue;
            }
            self.month = 1;
            self.year += 1;
        }
    }

    pub fn add_months(&mut self, months: usize) {
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

    pub fn add_days(&mut self, days: usize) {
        let mut days_left = days;
        loop {
            let mut month_length = MONTH_LENGTHS[self.month];
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

    pub fn add_hours(&mut self, hours: usize) {
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

    pub fn add_minutes(&mut self, minutes: usize) {
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

    pub fn add_seconds(&mut self, seconds: usize) {
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