use std::fs;

// This Works but part A was 8s and part B is even more data
// needs a better algo

pub fn main() {
    let contents = fs::read_to_string("./resources/day_12").expect("Should have been able to read the file");
    let mut lines: Vec<&str> = contents.split("\n").collect();

    let mut sum: u64 = 0;
    for line in lines {
        let sep: Vec<&str> = line.split(" ").collect();
        let conditions: Vec<char> = sep[0].chars().collect();
        let numbers: Vec<u64> = sep[1].split(",").map(|a| str_to_u64(a)).collect();
        sum += permutate(0, conditions, numbers);
    }
    println!("Sum: {}", sum);
}

fn permutate(step: usize, mut conditions: Vec<char>, numbers: Vec<u64>) -> u64 {
    let mut a= 0;
    if step == conditions.len() {
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

    // // get all finished # blocks
    // let mut local_numbers: Vec<(char, u64)> = Vec::new();
    // let mut last = 0;
    // for con in &conditions {
    //     match con {
    //         '#' => {
    //             if local_numbers.len() == 0 {
    //                 local_numbers.push(('#', 0));
    //             }
    //             if local_numbers[last].0 == '#' {
    //                 local_numbers[last].1 += 1;
    //             }else {
    //                 local_numbers.push(('#', 1));
    //                 last += 1;
    //             }
    //         },
    //         '?' => {
    //             if local_numbers.len() == 0 {
    //                 local_numbers.push(('?', 0));
    //             }
    //             if local_numbers[last].0 == '?' {
    //                 local_numbers[last].1 += 1;
    //             }else {
    //                 local_numbers.push(('?', 1));
    //                 last += 1;
    //             }
    //         },
    //         _ => {  },
    //     }
    // }
    // // println!("{:?} ?? {:?}", local_numbers, numbers);
    // if local_numbers.len() > 0 {
    //     for i in 0..numbers.len() {
    //         if i < local_numbers.len() {
    //             if local_numbers[i].0 == '?' {
    //                 break;
    //             }
    //             if local_numbers[i].1 > numbers[i] {
    //                 println!("ret: {:?} > {:?} ({:?})", local_numbers, numbers, conditions);
    //                 return 0;
    //             }
    //         }
    //     }
    // }

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