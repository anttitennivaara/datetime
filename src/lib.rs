pub mod datetime_struct;
pub mod time_struct;
pub mod functions;

#[cfg(test)]
mod tests {
    #[test]
    fn time_struct_test() {
        use crate::time_struct::build_time;
        let mut test_time = build_time(1_234_456, "nanoseconds");
        assert_eq!(test_time.to_unit("microseconds"), 1234);
        test_time.add_time(1, "seconds");
        assert_eq!(test_time.to_unit("microseconds"), 2234);
        for _ in 0..1_000_000_000 {
            println!("{}", test_time.to_string());
            test_time.add_time(10, "microseconds");
        }
    }
}