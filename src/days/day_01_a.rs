use std::fs;

pub fn main() {
    let contents = fs::read_to_string("./resources/day_01").expect("Should have been able to read the file");
    
    let mut cnt: u64 = 0;
    for line in contents.split("\n") {
        cnt += get_number(line);
    }
    println!("C: {}", cnt);
}

fn get_number(line: &str) -> u64 {

    let chars: Vec<char> = line.chars().collect();
    let mut ret: u64 = 0;
    let zero: u64 = '0' as u64;

    for c in chars.clone() {
        if c.is_digit(10) {
            ret = (c as u64 - zero) * 10;
            break;
        }
    }
    
    for c in chars.iter().rev() {
        if c.is_digit(10) {
            ret += *c as u64 - zero;
            break;
        }
    }
    ret
}