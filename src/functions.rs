pub const MONTH_LENGTHS: [usize; 13] = [0, 31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

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