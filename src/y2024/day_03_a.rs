extern crate regex;

use std::fs;
use regex::Regex;

pub fn main() {
    let contents = fs::read_to_string("./src/y2024/resources/day_03").expect("Should have been able to read the file");

    let regex: Regex = Regex::new(
        r#"mul\([0-9]{1,3},[0-9]{1,3}\)"#
    ).unwrap();

    let mut count = 0;
    for find in regex.find_iter(contents.as_str()) {
        let mut mul: String = find.as_str().to_string();
        mul = mul.replace("mul(", "");
        mul = mul.replace(")", "");
        let lists: Vec<&str> = mul.split(",").collect::<Vec<&str>>();
        if lists.len() == 2 {
            let numbers: Vec<u32> = lists.iter().map(|a| a.parse::<u32>().unwrap()).collect();
            count += numbers[0] * numbers[1];
        }
    }

    println!("C: {}", count);
}