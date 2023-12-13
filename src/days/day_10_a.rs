use std::fs;

pub fn main() {
    let contents = fs::read_to_string("./resources/day_10").expect("Should have been able to read the file");
    let mut lines: Vec<&str> = contents.split("\n").collect();

}