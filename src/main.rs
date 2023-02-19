pub mod datetime_struct;
pub mod functions;

use crate::datetime_struct::DateTime;

fn main() {
    let start = DateTime {
        year: 2001,
        month: 12,
        day: 24,
        hour: 8,
        minute: 15,
        second: 0,
    };

    let end = DateTime {
        year: 2023,
        month: 2,
        day: 19,
        hour: 9,
        minute: 24,
        second: 0,
    };

    println!("{}", (end - start).to_units_string());
}