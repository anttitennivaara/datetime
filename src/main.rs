const MONTH_LENGTHS: [usize; 13] = [0, 31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

pub fn is_leap_year(year: usize) -> bool {
    if year % 4 != 0 {
        return false;
    }
    if year % 100 == 0 && year % 400 != 0 {
        return false;
    }
    true
}

// Conversions to smaller unit:
pub fn minutes_to_seconds(minutes: usize) -> usize {
    minutes * 60
}
pub fn hours_to_minutes(hours: usize) -> usize {
    hours * 60
}
pub fn hours_to_seconds(hours: usize) -> usize {
    minutes_to_seconds(hours_to_minutes(hours))
}
pub fn days_to_hours(days: usize) -> usize {
    days * 24
}
pub fn days_to_minutes(days: usize) -> usize {
    hours_to_minutes(days_to_hours(days))
}
pub fn days_to_seconds(days: usize) -> usize {
    minutes_to_seconds(days_to_minutes(days))
}
pub fn months_to_days(first_month: usize, last_month: usize, year: usize) -> usize {
    let mut days: usize = 0;
    for month in first_month..last_month {
        days += &MONTH_LENGTHS[month];
    }
    if is_leap_year(year) {
        days += 1;
    }
    days
}
pub fn years_to_days(first_year: usize, last_year: usize) -> usize {
    let mut days: usize = 0;
    for year in first_year..last_year {
        days += 365;
        if is_leap_year(year) {
            days += 1;
        }
    }
    days
}

// Conversions to larger unit:
pub fn hours_to_days(hours: usize) -> usize {
    hours / 24
}
pub fn minutes_to_hours(minutes: usize) -> usize {
    minutes / 60
}
pub fn minutes_to_days(minutes: usize) -> usize {
    hours_to_days(minutes_to_hours(minutes))
}
pub fn seconds_to_minutes(seconds: usize) -> usize {
    seconds / 60
}
pub fn seconds_to_hours(seconds: usize) -> usize {
    minutes_to_hours(seconds_to_minutes(seconds))
}
pub fn seconds_to_days(seconds: usize) -> usize {
    hours_to_days(seconds_to_hours(seconds))
}

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

fn main() {
    let start = DateTime {
        year: 2022,
        month: 1,
        day: 1,
        hour: 0,
        minute: 0,
        second: 0,
    };

    let current = DateTime {
        year: 2022,
        month: 11,
        day: 15,
        hour: 3,
        minute: 13,
        second: 0,
    };

    let end = DateTime {
        year: 2023,
        month: 1,
        day: 1,
        hour: 0,
        minute: 0,
        second: 0,
    };

    let start_unix = start.to_unix_time();
    let current_unix = current.to_unix_time();
    let end_unix = end.to_unix_time();

    let progress: f64 =
        (((current_unix - start_unix) as f64) / ((end_unix - start_unix) as f64)) * 100.0;

    println!("{} %", progress);
}