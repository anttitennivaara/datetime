pub mod datetime_struct;
pub mod functions;

use crate::datetime_struct::DateTime;

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