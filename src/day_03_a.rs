use std::fs;

pub fn main() {
    let contents = fs::read_to_string("./resources/day_03").expect("Should have been able to read the file");

    let mut matrix: Vec<Vec<char>> = Vec::new();

    for line in contents.split("\n") {
        let chars: Vec<char> = line.chars().collect();
        matrix.push(chars);
    }
    
    let cnt: u64 = get_sum(matrix);
    println!("C: {}", cnt);
}

fn get_sum(matrix: Vec<Vec<char>>) -> u64 {
    let mut sum: u64 = 0;
    
    let mut current_number: Vec<char> = Vec::new();
    let mut start: usize = 0;
    let mut end: usize = 0;

    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            match matrix[i][j] {
                '0'..='9' => {
                    if current_number.len() == 0 {
                        start = j;
                    }
                    current_number.push(matrix[i][j]);
                },
                _ => {
                    if current_number.len() != 0 {
                        end = j;

                        if check_hit(matrix.clone(), i, start, end) {
                            sum += vec_to_number(current_number);
                        }
                        // werte zurücksetzten, alle
                        current_number = Vec::new();
                        start = 0;
                        end = 0;
                    }
                }
            }
        }
        if current_number.len() > 0 {
            if check_hit(matrix.clone(), i, start, matrix[i].len()) {
                sum += vec_to_number(current_number);
            }
            current_number = Vec::new();
            start = 0;
            end = 0;
        }
    }
    return sum;
}

fn check_hit(matrix: Vec<Vec<char>>, i: usize, mut start: usize, mut end: usize) -> bool {
    if start > 0 {
        start -= 1;
    }
    if end + 1 < matrix[i].len() {
        end += 1;
    }
    for n in start..end {
        // above
        if i > 0 && !matrix[i - 1][n].is_digit(10) && matrix[i - 1][n] != '.' {
            return true;
        }
        // middle
        if !matrix[i][n].is_digit(10) && matrix[i][n] != '.' {
            return true;
        }
        // under
        if i + 1 < matrix.len() && !matrix[i + 1][n].is_digit(10) && matrix[i + 1][n] != '.' {
            return true;
        }
    }
    return false;
}

fn vec_to_number(current_number: Vec<char>) -> u64 {
    let mut cnt: u32 = 0;
    let mut number: u64 = 0;
    for n in (0..current_number.len()).rev() {
        number += (current_number[n] as u64 - '0' as u64) * u64::pow(10, cnt);
        cnt += 1;
    }
    return number;
}