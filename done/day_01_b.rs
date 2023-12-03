use std::fs;
use std::collections::HashMap;

pub fn main() {
    let contents = fs::read_to_string("./resources/day_01").expect("Should have been able to read the file");

    let mut cnt: u64 = 0;
    for line in contents.split("\n") {
        let res = get_number(line);
        println!("{}", res);
        cnt += res;
    }
    println!("C: {}", cnt);
}

fn get_number(line: &str) -> u64 {
    let chars: Vec<char> = line.chars().collect();

    let mut ret_vec: Vec<u64> = Vec::new();
    for index in 0..chars.len() {
        match chars[index] {
            '0'..='9' => {
                ret_vec.push(chars[index] as u64 - '0' as u64);
            },
            'o' => {
                if chars.len() > index + 2 && chars[index + 1] == 'n' && chars[index + 2] == 'e' {
                    ret_vec.push(1);
                }
            },
            't' => {
                if chars.len() > index + 2 && chars[index + 1] == 'w' && chars[index + 2] == 'o' {
                    ret_vec.push(2);
                }
                if chars.len() > index + 4 && chars[index + 1] == 'h' && chars[index + 2] == 'r' && chars[index + 3] == 'e' && chars[index + 4] == 'e' {
                    ret_vec.push(3);
                }
            },
            'f' => {
                if chars.len() > index + 3 && chars[index + 1] == 'o' && chars[index + 2] == 'u' && chars[index + 3] == 'r' {
                    ret_vec.push(4);
                }
                if chars.len() > index + 3 && chars[index + 1] == 'i' && chars[index + 2] == 'v' && chars[index + 3] == 'e' {
                    ret_vec.push(5);
                }
            },
            's' => {
                if chars.len() > index + 2 && chars[index + 1] == 'i' && chars[index + 2] == 'x' {
                    ret_vec.push(6);
                }
                if chars.len() > index + 4 && chars[index + 1] == 'e' && chars[index + 2] == 'v' && chars[index + 3] == 'e' && chars[index + 4] == 'n' {
                    ret_vec.push(7);
                }
            },
            'e' => {
                if chars.len() > index + 4 && chars[index + 1] == 'i' && chars[index + 2] == 'g' && chars[index + 3] == 'h' && chars[index + 4] == 't' {
                    ret_vec.push(8);
                }
            },
            'n' => {
                if chars.len() > index + 3 && chars[index + 1] == 'i' && chars[index + 2] == 'n' && chars[index + 3] == 'e' {
                    ret_vec.push(9);
                }
            },
            _ => { },
        
        }
    }
    return ret_vec[0] * 10 + ret_vec[ret_vec.len() - 1];
}