use std::{fs, io};

pub fn read_day(day: &str) -> io::Result<String> {
    fs::read_to_string(format!("./data/day{}", day))
}

pub fn read_day_example(day: &str) -> io::Result<String> {
    fs::read_to_string(format!("./data/day{}_example", day))
}
