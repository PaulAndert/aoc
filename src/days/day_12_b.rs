use std::fs;

// This Works but part A was 8s and part B is even more data
// needs a better algo

pub fn main() {
    let contents = fs::read_to_string("./resources/day_12").expect("Should have been able to read the file");
    let mut lines: Vec<&str> = contents.split("\n").collect();

    let mut sum: u64 = 0;
    for line in lines {
    // let line = lines[5];
        let sep: Vec<&str> = line.split(" ").collect();
        let conditions: Vec<char> = sep[0].chars().collect();
        let numbers: Vec<u64> = sep[1].split(",").map(|a| str_to_u64(a)).collect();

        let mut conditions_big: Vec<char> = conditions.clone();
        let mut numbers_big: Vec<u64> = numbers.clone();
        // multiply
        for _i in 0..4 {
            conditions_big.push('?');
            for j in 0..conditions.len() {
                conditions_big.push(conditions[j]);
            }
            for j in 0..numbers.len() {
                numbers_big.push(numbers[j]);
            }
        }
        // println!("C: {:?}", conditions_big.clone());
        // println!("CL: {:?}", conditions_big.len());
        // println!("N: {:?}", numbers_big.clone());
        // println!("NL: {:?}", numbers_big.len());

        let qs_left: usize = conditions_big.iter().map(|c| if *c == '?' { 1 } else { 0 }).sum();

        sum += permutate(0, qs_left, conditions_big, numbers_big, false);
    }
    println!("Sum: {}", sum);
}

fn permutate(step: usize, qs_left: usize, mut conditions: Vec<char>, numbers: Vec<u64>, check: bool) -> u64 {
    let mut a= 0;
    // println!("{}: {:?}", step, conditions);

    if check {
        if !check_grouping(step, conditions.clone(), numbers.clone()) {
            return 0;
        }
    }


    if step == conditions.len() || qs_left == 0 {
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
        if local_numbers == numbers {
            return 1;
        }else {
            return 0;
        }
    }

    if conditions[step] == '?' {
        conditions[step] = '#';
        a += permutate(step + 1, qs_left - 1, conditions.clone(), numbers.clone(), true);
        conditions[step] = '.';
        a += permutate(step + 1, qs_left - 1, conditions, numbers, true);
    }else {
        a += permutate(step + 1, qs_left, conditions, numbers, true);
    }
    a
}

fn check_grouping(end: usize, conditions: Vec<char>, numbers: Vec<u64>) -> bool {
    let mut size: u64 = 0;
    let mut index: usize = 0;

    for i in 0..end {
        if conditions[i] == '#' {
            size += 1;
        }else {
            if index >= numbers.len() {
                for j in i..end {
                    if conditions[j] == '#' {
                        // println!("FB: {:?} -> {:?}", conditions, numbers);
                        return false;
                    }
                }
                return true;
            }
            if size > 0 {
                if numbers[index] != size {
                    // println!("FC: {:?} -> {:?}", conditions, numbers);
                    return false;
                }else {
                    index += 1;
                    size = 0;
                }
            }
        }
    }
    // println!("T: {:?}", conditions);
    true
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