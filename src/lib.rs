pub mod datetime_struct;
pub mod time_struct;
pub mod functions;

#[cfg(test)]
mod tests {
    #[test]
    fn time_struct_test() {
        use crate::time_struct::build_time;
        let mut test_time = build_time(1_000_000, "nanoseconds");
        assert_eq!(test_time.to_unit("microseconds"), 1000);
        test_time.add_time(1, "seconds");
        assert_eq!(test_time.to_unit("microseconds"), 2000);
    }
}