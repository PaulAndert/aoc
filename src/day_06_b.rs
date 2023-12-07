use std::fs;

pub fn main() {
    let contents = fs::read_to_string("./resources/day_06").expect("Should have been able to read the file");
    let lines: Vec<&str> = contents.split("\n").collect();

    let time_line: &str = &lines[0].replace(" ", "");
    let time: u64 = str_to_u64(time_line.split(":").last().unwrap());

    let distance_line: &str = &lines[1].replace(" ", "");
    let distance: u64 = str_to_u64(distance_line.split(":").last().unwrap());

    let mut all: u64 = 0;
    for j in 0..=time {
        let drive = time - j;
        let speed = j;
        let local_distance = drive * speed;

        if local_distance > distance {
            all += 1;
        }
    }

    println!("C: {}", all);
}

fn str_to_u64(string_number: &str) -> u64 {
    let chars: Vec<char> = string_number.chars().collect();
    let mut cnt: u32 = 0;
    let mut number: u64 = 0;
    for n in (0..chars.len()).rev() {
        number += (chars[n] as u64 - '0' as u64) * u64::pow(10, cnt);
        cnt += 1;
    }
    return number;
}