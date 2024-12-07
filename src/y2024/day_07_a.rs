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
    let ops: u32 = numbers.len() as u32 - 2;
    let mut size: usize = 2_usize.pow(ops);
    let mut ops_vec: Vec<Vec<bool>> = vec![Vec::new(); size];

    let mut times: usize = 2;
    let mut current_plus: bool = true;

    size /= 2;

    while size >= 1 {
        let mut loc_idx: usize = 0;
        for _t in 0..times {
    
            for _s in 0..size {
                ops_vec[loc_idx].push(current_plus);
                loc_idx += 1;
            }
            current_plus = !current_plus;
        }
        size /= 2;
        times *= 2;
    }

    for ops in ops_vec {
        let mut value: u64 = numbers[1];
        for op_idx in 0..ops.len() {
            if ops[op_idx] {
                value += numbers[op_idx + 2];
            } else {
                value *= numbers[op_idx + 2];
            }
        }
        if value == numbers[0] {
            return true;
        }
    }
    return false;
}
