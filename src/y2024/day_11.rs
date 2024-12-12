use std::time::Instant;
use std::fs;

pub fn main_a() {
    let contents = fs::read_to_string("./src/y2024/resources/day_11").expect("Should have been able to read the file");

    let mut numbers: Vec<String> = Vec::new();
    for num in contents.split(" ") {
        match num.parse::<String>() { Ok(mut a) => {
            if a.contains("\n") {
                a = a.replace("\n", "");
            }
            numbers.push(a)
        }, Err(_e) => {}};
    }

    let max = 25;
    let mut loop_idx = 0;
    while loop_idx < max {
    
        let mut new_numbers: Vec<String> = Vec::new();
        for idx in 0..numbers.len() {
            let current: String = numbers[idx].clone();
            let size: u32 = current.len() as u32;
            if current.eq("0") {
                new_numbers.push(String::from("1"));
            }else if size % 2 == 0 {
                let base:(&str, &str) = current.split_at((size / 2) as usize);
                new_numbers.push(remove_leading_zeros(base.0));
                new_numbers.push(remove_leading_zeros(base.1));
            }else {
                new_numbers.push(multiply_by_2024(current));
            }
        }

        numbers = new_numbers;
        loop_idx += 1;
    }

    println!("C: {}", numbers.len());
}

pub fn main_b() {
    let contents = fs::read_to_string("./src/y2024/resources/day_11").expect("Should have been able to read the file");
    // let contents = "0";
    let mut cnt: u32 = 0;

    let mut numbers: Vec<String> = Vec::new();
    for num in contents.split(" ") {
        match num.parse::<String>() { Ok(mut a) => {
            if a.contains("\n") {
                a = a.replace("\n", "");
            }
            numbers.push(a)
        }, Err(_e) => {}};
    }

    for n in numbers {
        let now = Instant::now();
        cnt += calculate_size(n.clone(), 0, 25);
        println!("{}: {:.2?}", n, now.elapsed());
    }

    // println!("{:?}", numbers);


    println!("C: {}", cnt);
}

fn calculate_size(input: String, step: usize, max: usize) -> u32 {
    // println!("\n{} {}", step, input);
    if step >= max {
        return 1;
    }
    let size: usize = input.len(); 
    if input == "0" {
        // TODO this step is 100% calculatable, dont know how
        if step + 3 <= max {
            let tmp = calculate_size(String::from("20"), step + 3, max);
            return tmp + calculate_size(String::from("24"), step + 3, max);
        }else {
            return calculate_size(String::from("1"), step + 1, max);
        }
    }else if size % 2 == 0 {
        // println!("even");
        let base:(&str, &str) = input.split_at((size / 2) as usize);
        let tmp = calculate_size(String::from(base.0), step + 1, max);
        return tmp + calculate_size(remove_leading_zeros(base.1), step + 1, max);
    }else {
        // println!("odd");
        return calculate_size(multiply_by_2024(input), step + 1, max);
    }
}

fn remove_leading_zeros(input: &str) -> String {
    let output = String::from(input.trim_start_matches('0'));
    if output == "" {
        return String::from("0");
    }
    return output;
}

fn multiply_by_2024(number: String) -> String {
    
    // pultiply by 2
    let mut new_numbers_2: Vec<u8> = Vec::new();
    let mut new_numbers_4: Vec<u8> = Vec::new();
    for num in number.chars().rev() {
        let num_08: u8 = num as u8 - 48;
        new_numbers_2.push(num_08 * 2);
        new_numbers_4.push(num_08 * 4);
    }

    let mut new_numbers_2_single: Vec<u8> = Vec::new();
    let mut new_numbers_4_single: Vec<u8> = Vec::new();
    let mut carry: u8 = 0;
    for idx in 0..new_numbers_2.len() {
        let mut current: u8 = new_numbers_2[idx];
        if carry != 0 {
            current += carry;
            carry = 0;
        }
        if current > 9 {
            carry = current / 10;
            new_numbers_2_single.push(current % 10);
        }else {
            new_numbers_2_single.push(current);
        }
    }
    if carry != 0 {
        new_numbers_2_single.push(carry);
        carry = 0;
    }
    for idx in 0..new_numbers_4.len() {
        let mut current: u8 = new_numbers_4[idx];
        if carry != 0 {
            current += carry;
            carry = 0;
        }
        if current > 9 {
            carry = current / 10;
            new_numbers_4_single.push(current % 10);
        }else {
            new_numbers_4_single.push(current);
        }
    }
    if carry != 0 {
        new_numbers_4_single.push(carry);
    }

    // println!("2: {:?}", new_numbers_2_single);
    // println!("4: {:?}", new_numbers_4_single);

    let mut final_number: Vec<u8> = vec![0; 4];
    let mut idx: usize = 0;
    let mut carry_1 = 0;
    let mut carry_3 = 0;
    let mut carry_4 = 0;
    while idx <= new_numbers_4_single.len() {

        // 202_4_
        if idx < new_numbers_4_single.len() {
            let loc_4 = new_numbers_4_single[idx];
            let loc_idx = idx;

            if loc_idx < final_number.len() {
                let temp = carry_1 + final_number[loc_idx] + loc_4;
                if temp > 9 {
                    carry_1 = temp / 10;
                    final_number[loc_idx] = temp % 10;
                }else {
                    final_number[loc_idx] = temp;
                }
            }else {
                let temp = carry_1 + loc_4;
                carry_1 = 0;
                if temp > 9 {
                    carry_1 = temp / 10;
                    final_number.push(temp % 10);
                }else {
                    final_number.push(temp);
                }
            }
        }else if idx == new_numbers_4_single.len() {
            let loc_idx = idx;
            if loc_idx < final_number.len() {
                let temp = carry_1 + final_number[loc_idx];
                if temp > 9 {
                    carry_1 = temp / 10;
                    final_number[loc_idx] = temp % 10;
                }else {
                    carry_1 = 0;
                    final_number[loc_idx] = temp;
                }
            }else {
                let temp = carry_1;
                if temp > 9 {
                    carry_1 = temp / 10;
                    final_number.push(temp % 10);
                }else {
                    carry_1 = 0;
                    final_number.push(temp);
                }
            }
        }

        idx += 1
    }

    // println!("F: {:?}", final_number.clone());

    idx = 0;
    while idx <= new_numbers_2_single.len() {
        // println!("{}: {:?}", idx, final_number);

        if idx < new_numbers_2_single.len() {
            // println!("A");
            let loc_2 = new_numbers_2_single[idx];
            let loc_idx = idx + 1;
            if loc_idx < final_number.len() {
                // println!("A.A {}", carry_3);
                let temp = carry_3 + final_number[loc_idx] + loc_2;
                if temp > 9 {
                    carry_3 = temp / 10;
                    final_number[loc_idx] = temp % 10;
                }else {
                    carry_3 = 0;
                    final_number[loc_idx] = temp;
                }
            }else {
                // println!("A.B");
                let temp = carry_3 + loc_2;
                if temp > 9 {
                    carry_3 = temp / 10;
                    final_number.push(temp % 10);
                }else {
                    carry_3 = 0;
                    final_number.push(temp);
                }
            }
        }else if idx == new_numbers_2_single.len() && carry_3 != 0 {
            // println!("B");
            let loc_idx = idx + 1;
            if loc_idx < final_number.len() {
                let temp = carry_3 + final_number[loc_idx];
                if temp > 9 {
                    carry_3 = temp / 10;
                    final_number[loc_idx] = temp % 10;
                }else {
                    final_number[loc_idx] = temp;
                }
            }else {
                let temp = carry_3;
                carry_3 = 0;
                if temp > 9 {
                    carry_3 = temp / 10;
                    final_number.push(temp % 10);
                }else {
                    final_number.push(temp);
                }
            }
        }
        idx += 1;
    }

    // println!("F: {:?}", final_number.clone());


    idx = 0;
    while idx <= new_numbers_2_single.len() {
        // println!("{}: {:?}", idx, final_number);

        if idx < new_numbers_2_single.len() {
            // println!("A");
            let loc_2 = new_numbers_2_single[idx];
            let loc_idx = idx + 3;
            if loc_idx < final_number.len() {
                // println!("A.A {}", carry_4);
                let temp = carry_4 + final_number[loc_idx] + loc_2;
                if temp > 9 {
                    carry_4 = temp / 10;
                    final_number[loc_idx] = temp % 10;
                }else {
                    carry_4 = 0;
                    final_number[loc_idx] = temp;
                }
            }else {
                // println!("A.B");
                let temp = carry_4 + loc_2;
                if temp > 9 {
                    carry_4 = temp / 10;
                    final_number.push(temp % 10);
                }else {
                    carry_4 = 0;
                    final_number.push(temp);
                }
            }
        }else if idx == new_numbers_2_single.len() && carry_4 != 0 {
            // println!("B");
            let loc_idx = idx + 3;
            if loc_idx < final_number.len() {
                let temp = carry_4 + final_number[loc_idx];
                if temp > 9 {
                    carry_4 = temp / 10;
                    final_number[loc_idx] = temp % 10;
                }else {
                    final_number[loc_idx] = temp;
                }
            }else {
                let temp = carry_4;
                carry_4 = 0;
                if temp > 9 {
                    carry_4 = temp / 10;
                    final_number.push(temp % 10);
                }else {
                    final_number.push(temp);
                }
            }
        }
        idx += 1;
    }

    // println!("F: {:?}", final_number.clone());

    final_number.iter().rev().map(|c| (c + 48) as char).collect::<String>()
    // char_vector.iter().cloned().collect::<String>();
}