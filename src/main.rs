pub mod datetime_struct;
pub mod functions;

use crate::datetime_struct::DateTime;

fn main() {
    let mut date = DateTime {
        year: 2001,
        month: 12,
        day: 24,
        hour: 8,
        minute: 15,
        second: 0,
    };

    date.add_months(-12);

    println!("{}.{}.{}", date.day, date.month, date.year);
}