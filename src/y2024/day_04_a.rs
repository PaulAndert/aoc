extern crate regex;

use std::fs;
use regex::Regex;

pub fn main() {
    let contents = fs::read_to_string("./src/y2024/resources/day_03").expect("Should have been able to read the file");

    let regex: Regex = Regex::new(
        r#"mul\([0-9]{1,3},[0-9]{1,3}\)|do\(\)|don't\(\)"#
    ).unwrap();

    let mut count = 0;
    let mut active: bool = true;

    for find in regex.find_iter(contents.as_str()) {
        let mut operation: String = find.as_str().to_string();

        if operation.contains("do") {
            if operation.contains("don't") {
                active = false;
            }else {
                active = true;
            }
        }

        if active {
            operation = operation.replace("mul(", "");
            operation = operation.replace(")", "");
            let lists: Vec<&str> = operation.split(",").collect::<Vec<&str>>();
            if lists.len() == 2 {
                let numbers: Vec<u32> = lists.iter().map(|a| a.parse::<u32>().unwrap()).collect();
                count += numbers[0] * numbers[1];
            }
        }
    }

    println!("C: {}", count);
}