use std::fs;
use std::process::exit;

pub fn main() {
    let contents = fs::read_to_string("./resources/day_12").expect("Should have been able to read the file");
    let mut lines: Vec<&str> = contents.split("\n").collect();

    let mut sum: u64 = 0;
    let mut ccc: u64 = 0;
    for line in lines {
        let sep: Vec<&str> = line.split(" ").collect();
        let conditions: Vec<char> = sep[0].chars().collect();
        let numbers: Vec<u64> = sep[1].split(",").map(|a| str_to_u64(a)).collect();

        // println!("{:?}", conditions);

        let a = permutate(0, conditions, numbers);
        println!("A: {}", a);
        sum += a;
        ccc += 1;
        // if ccc == 2 {
        //     break;
        // }
    }
    println!("Sum: {}", sum);
}

fn permutate(step: usize, mut conditions: Vec<char>, numbers: Vec<u64>) -> u64 {
    let mut a= 0;
    if step == conditions.len() {
        // check and return 0 or 1
        // println!("CC: {:?}", conditions);
        let mut local_numbers: Vec<u64> = vec![0];
        for con in conditions {
            let last = local_numbers.len() - 1;
            if con == '#' {
                local_numbers[last] += 1;
            }else if local_numbers[last] != 0 {
                local_numbers.push(0);
            }
        }
        let last = local_numbers.len() - 1;
        if local_numbers[last] == 0 {
            local_numbers.remove(last);
        }
        // println!("LL: {:?}", local_numbers);
        // exit(0);
        if local_numbers == numbers {
            return 1;
        }else {
            return 0;
        }
    }
    if conditions[step] == '?' {
        conditions[step] = '.';
        a += permutate(step + 1, conditions.clone(), numbers.clone());
        conditions[step] = '#';
        a += permutate(step + 1, conditions, numbers);
    }else {
        a += permutate(step + 1, conditions, numbers);
    }
    a
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