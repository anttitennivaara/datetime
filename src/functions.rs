pub fn get_month_length(month: isize, year: isize) -> isize {
    let mut lengths: [isize; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    if is_leap_year(year) {
        lengths[1] += 1;
    }
    lengths[(month as usize) - 1]
}

pub fn is_between(min: isize, max: isize, number: isize) -> bool {
    number >= min && number <= max
}

pub fn is_leap_year(year: isize) -> bool {
    if year % 4 != 0 {
        return false;
    }
    if year % 100 == 0 && year % 400 != 0 {
        return false;
    }
    true
}

// Conversions to smaller unit:
pub fn minutes_to_seconds(minutes: isize) -> isize {
    minutes * 60
}
pub fn hours_to_minutes(hours: isize) -> isize {
    hours * 60
}
pub fn hours_to_seconds(hours: isize) -> isize {
    minutes_to_seconds(hours_to_minutes(hours))
}
pub fn days_to_hours(days: isize) -> isize {
    days * 24
}
pub fn days_to_minutes(days: isize) -> isize {
    hours_to_minutes(days_to_hours(days))
}
pub fn days_to_seconds(days: isize) -> isize {
    minutes_to_seconds(days_to_minutes(days))
}
pub fn months_to_days(first_month: isize, last_month: isize, year: isize) -> isize {
    let mut days: isize = 0;
    for month in first_month..last_month {
        days += get_month_length(month, year);
    }
    days
}
pub fn years_to_days(first_year: isize, last_year: isize) -> isize {
    let mut days: isize = 0;
    for year in first_year..last_year {
        days += 365;
        if is_leap_year(year) {
            days += 1;
        }
    }
    days
}

// Conversions to larger unit:
pub fn hours_to_days(hours: isize) -> isize {
    hours / 24
}
pub fn minutes_to_hours(minutes: isize) -> isize {
    minutes / 60
}
pub fn minutes_to_days(minutes: isize) -> isize {
    hours_to_days(minutes_to_hours(minutes))
}
pub fn seconds_to_minutes(seconds: isize) -> isize {
    seconds / 60
}
pub fn seconds_to_hours(seconds: isize) -> isize {
    minutes_to_hours(seconds_to_minutes(seconds))
}
pub fn seconds_to_days(seconds: isize) -> isize {
    hours_to_days(seconds_to_hours(seconds))
}

pub fn format_string(number: isize) -> String {
    if number < 10 {
        let mut number_string = String::from("0");
        number_string.insert_str(1, number.to_string().as_str());
        return number_string;
    }
    number.to_string()
}