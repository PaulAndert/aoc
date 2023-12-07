use std::fs;

pub fn main() {
    let contents = fs::read_to_string("./resources/day_05").expect("Should have been able to read the file");
    let mut lines: Vec<&str> = contents.split("\n").collect();

    let seed_parts: Vec<&str> = lines[0].split(": ").collect();
    lines.remove(0);
    let mut seeds: Vec<u64> = seed_parts[1].split(" ").map(|s| str_to_number(s)).collect();

    // stages :: lines :: 3 numbers
    let mut cube: Vec<Vec<Vec<u64>>> = vec![Vec::new()];

    let mut stage: usize = 0;
    for line in lines {
        match line.chars().next() {
            Some('0'..='9') => {
                let numbers: Vec<u64> = line.split(" ").map(|s| str_to_number(s)).collect();
                cube[stage].push(numbers);
            },
            _ => {
                if line != "" {
                    stage += 1;
                    cube.push(Vec::new());
                }
            },
        }
    }

    for step in cube {
        for idx in 0..seeds.len() {
            let seed: u64 = seeds[idx];
            // search match
            let mut new_dest: u64 = 0;
            for arr in &step {
                let bottom = arr[1];
                let top = arr[1] + arr[2] - 1;
                if seed >= bottom && seed <= top {
                    let diff = seed - arr[1];
                    new_dest = arr[0] + diff;
                }
            }
            seeds[idx] = if new_dest == 0 {
                seed
            } else {
                new_dest
            };
        }
    }
    match seeds.iter().min() {
        Some(min) => {
            println!("C: {}", min);
        },
        None => {
            println!("Error");
        },
    }
}

fn str_to_number(string_number: &str) -> u64 {
    let chars: Vec<char> = string_number.chars().collect();
    let mut cnt: u32 = 0;
    let mut number: u64 = 0;
    for n in (0..chars.len()).rev() {
        number += (chars[n] as u64 - '0' as u64) * u64::pow(10, cnt);
        cnt += 1;
    }
    return number;
}