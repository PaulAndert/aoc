use std::fs;

pub fn main() {
    let contents = fs::read_to_string("./resources/day_06").expect("Should have been able to read the file");
    let lines: Vec<&str> = contents.split("\n").collect();

    let mut times_str: Vec<&str> = lines[0].split(" ").filter(|&x| !x.is_empty()).collect();
    times_str.remove(0);
    let times: Vec<u64> = times_str.iter().map(|x| str_to_u64(x)).collect();
    let mut distances_str: Vec<&str> = lines[1].split(" ").filter(|&x| !x.is_empty()).collect();
    distances_str.remove(0);
    let distances: Vec<u64> = distances_str.iter().map(|x| str_to_u64(x)).collect();

    let mut all: Vec<u64> = Vec::new();
    for i in 0..distances.len() {
        all.push(0);
        for j in 0..=times[i] {
            let drive = times[i] - j;
            let speed = j;
            let distance = drive * speed;

            if distance > distances[i] {
                all[i] += 1;
            }
        }
    }
    let mut product: u64 = 1;
    for i in all {
        product *= i;
    }
    println!("C: {}", product);
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