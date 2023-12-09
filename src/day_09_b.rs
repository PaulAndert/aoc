use std::fs;

pub fn main() {
    let contents = fs::read_to_string("./resources/day_09").expect("Should have been able to read the file");
    let mut lines: Vec<&str> = contents.split("\n").collect();

    let mut sum: i64 = 0;

    let mut ccc = 0;

    for line in lines {
        // get the differences
        let mut all_numbers: Vec<Vec<i64>> = Vec::new();
        all_numbers.push(line.split(" ").map(|a| str_to_i64(a)).collect::<Vec<i64>>());
        while !check_zero(all_numbers[all_numbers.len() - 1].clone()) {
            let mut new_numbers: Vec<i64> = Vec::new();
            for i in 0..(all_numbers[all_numbers.len() - 1].len() - 1) {
                new_numbers.push(all_numbers[all_numbers.len() - 1][i + 1] - all_numbers[all_numbers.len() - 1][i]);
            }
            all_numbers.push(new_numbers);
        }
        // for r in &all_numbers {
        //     println!("{:?}", r);
        // }
        // extrapolate values
        for i in (0..all_numbers.len() - 1).rev() {
            if i == all_numbers.len() {
                all_numbers[i].push(0);
            }else {
                let new_value: i64 = all_numbers[i + 1][all_numbers[i + 1].len() - 1] + all_numbers[i][all_numbers[i].len() - 1];
                all_numbers[i].push(new_value);
            }
        }
        // println!("{:?}", all_numbers);
        sum += all_numbers[0][all_numbers[0].len() - 1];

        // if ccc == 1 { break; }
        // else { ccc += 1; }
    }
    println!("Sum: {}", sum);
}

fn check_zero(numbers: Vec<i64>) -> bool {
    let mut cnt: usize = 0;
    for i in &numbers {
        if *i == 0 {
            cnt += 1;
        }
    }
    return if cnt == numbers.len() {
        true
    } else {
        false
    }
}

fn str_to_i64(string_number: &str) -> i64 {
    let mut chars: Vec<char> = string_number.chars().collect();
    let negativ: bool = if chars[0] == '-' { chars.remove(0); true } else { false };

    let mut cnt: u32 = 0;
    let mut number: i64 = 0;
    for n in (0..chars.len()).rev() {
        number += (chars[n] as i64 - '0' as i64) * i64::pow(10, cnt);
        cnt += 1;
    }
    if negativ {
        return number * -1;
    }else {
        return number;
    }
}
