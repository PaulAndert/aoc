use std::fs;
use std::time::{Duration, Instant};

pub fn main() {
    let mut start = Instant::now();
    let contents = fs::read_to_string("./resources/day_05").expect("Should have been able to read the file");
    let mut lines: Vec<&str> = contents.split("\n").collect();

    let seed_parts: Vec<&str> = lines[0].split(": ").collect();
    lines.remove(0);
    let seeds: Vec<u64> = seed_parts[1].split(" ").map(|s| str_to_number(s)).collect();

    println!("Time: {:?}", start.elapsed());
    start = Instant::now();
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
    println!("Time: {:?}", start.elapsed());
    start = Instant::now();

    let mut min: u64 = u64::MAX;
    let mut db: bool = false;
    for idx in (0..seeds.len()).step_by(2) {
        start = Instant::now();


        let seed_start: u64 = seeds[idx];
        let seed_top: u64 = seeds[idx] + seeds[idx + 1];

        for seed in seed_start..seed_top {
            let mut value = seed;
            //println!("SSS: {} / {} ({}%)", seed, seed_top, (seed / seed_top) * 100);

            if (seed - seed_start) as f64 / (seed_top - seed_start) as f64 > 0.01 {
                println!("Time: {:?}", start.elapsed());
                println!("SSS: {} / {} ({:.2}%)", seed - seed_start, seed_top - seed_start, ((seed - seed_start) as f64 / (seed_top - seed_start) as f64) * 100.0);
                return ;
            }
            // if true { db = true; }
            // else { db = false; }
            // println!("NEW {}", value);
            for gardening_map in &cube {
                // println!("GM {:?}", gardening_map);
                for arr in gardening_map {
                    let bottom = arr[1];
                    let top = arr[1] + arr[2] - 1;
                    // if db { println!("{} .. {}", bottom, top); }
                    if value >= bottom && value <= top {
                        let diff = value - arr[1];
                        value = arr[0] + diff;
                        // if db { println!("V: {}", value); }
                        break;
                    }
                }
            }
            if value < min {
                min = value;
            }
            // println!("Time: {:?}", start.elapsed());
        }
    }
    println!("C: {}", min);

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