use std::fs;

pub fn main() {
    let contents = fs::read_to_string("./src/y2024/resources/day_07").expect("Should have been able to read the file");
    let mut cnt: u64 = 0;
    
    let mut numbers_vec: Vec<Vec<u64>> = Vec::new();

    for line in contents.split("\n") {
        let string_line: Vec<&str> = line.split(" ").collect();
        if string_line.len() <= 1 {
            continue;
        }
        let mut num_line: Vec<u64> = Vec::new();
        for idx in 0..string_line.len() {
            num_line.push(if idx == 0 {
                string_line[0].replace(":", "").parse::<u64>().unwrap()
            }else {
                string_line[idx].parse::<u64>().unwrap()
            });
        }
        numbers_vec.push(num_line);
    }

    for numbers in numbers_vec {
        let temp: u64 = numbers[0];
        if check(numbers) {
            cnt += temp;
        }
    }

    println!("C: {}", cnt);
}

fn check(numbers: Vec<u64>) -> bool {

    let op_options: usize = 3;
    let ops: u32 = numbers.len() as u32 - 2;
    let mut size: usize = op_options.pow(ops);
    let mut ops_vec: Vec<Vec<char>> = vec![Vec::new(); size];

    let mut times: usize = 3;
    let mut current_plus: char = 'P'; // _P_lus, _M_ultiplicate, _C_oncat

    size /= op_options;

    while size >= 1 {
        let mut loc_idx: usize = 0;
        for _t in 0..times {
    
            for _s in 0..size {
                ops_vec[loc_idx].push(current_plus);
                loc_idx += 1;
            }
            current_plus = match current_plus {
                'P' => 'M',
                'M' => 'C',
                'C' => 'P',
                _ => 'P'
            };
        }
        size /= op_options;
        times *= op_options;
    }

    for ops in ops_vec {
        let mut value: u64 = numbers[1];
        for op_idx in 0..ops.len() {
            match ops[op_idx] {
                'P' => value += numbers[op_idx + 2],
                'M' => value *= numbers[op_idx + 2],
                'C' => value = concatinate(value, numbers[op_idx + 2]),
                _ => panic!("Error")
            };
        }
        if value == numbers[0] {
            return true;
        }
    }
    return false;
}

fn concatinate(mut value: u64, new_number: u64) -> u64 {
    let base = new_number.checked_ilog10().unwrap_or(0) + 1;
    value *= 10_u64.pow(base);
    value += new_number;
    return value;
}