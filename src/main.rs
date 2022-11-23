pub mod datetime_struct;
pub mod functions;

use crate::datetime_struct::DateTime;

fn main() {
    let date = DateTime {
        year: 2001,
        month: 12,
        day: 24,
        hour: 8,
        minute: 15,
        second: 0,
    };

    let other = DateTime {
        year: 2022,
        month: 11,
        day: 21,
        hour: 22,
        minute: 15,
        second: 0,
    };

    println!("{}", (other.time_since(date, "days") as f32 / 365.25).to_string());
}