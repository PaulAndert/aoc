use std::fs;

pub fn main() {
    let contents = fs::read_to_string("./resources/day_03").expect("Should have been able to read the file");

    let mut matrix: Vec<Vec<char>> = Vec::new();

    for line in contents.split("\n") {
        let chars: Vec<char> = line.chars().collect();
        matrix.push(chars);
    }
    
    let cnt: u64 = get_gear_sum(matrix);
    println!("C: {}", cnt);
}

fn get_gear_sum(matrix: Vec<Vec<char>>) -> u64 {
    let mut sum: u64 = 0;
    let mut current_numbers: Vec<(usize, usize)> = Vec::new();

    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            match matrix[i][j] {
                '*' => {
                    // get boundaries
                    let top: i64 = if i > 0 { -1 } else { 0 };
                    let bottom: i64 = if i < matrix.len() { 1 } else { 0 };
                    let left: i64 = if j > 0 { -1 } else { 0 };
                    let right: i64 = if j < matrix[i].len() { 1 } else { 0 };

                    for a in top..=bottom {
                        for b in left..=right {
                            match matrix[(i as i64 + a) as usize][(j as i64 + b) as usize] {
                                '0'..='9' => {
                                    current_numbers.push(((i as i64 + a) as usize, (j as i64 + b) as usize));
                                },
                                _ => {},
                            }
                        }
                    }
                    if current_numbers.len() >= 2 {
                        sum += get_numbers(matrix.clone(), current_numbers);
                    }
                    current_numbers = Vec::new();
                },
                _ => {},
            }
        }
    }
    return sum;
}

fn get_numbers(matrix: Vec<Vec<char>>, current_numbers: Vec<(usize, usize)>) -> u64 {
    let mut starts: Vec<(usize, usize)> = Vec::new();
    // get starts of each numbers
    for current in current_numbers {
        let mut start: usize = 0;
        for j in (0..current.1).rev() {
            match matrix[current.0][j] {
                '0'..='9' => {
                    start = j;
                },
                _ => {
                    start = j + 1;
                    break;
                },
            }
        }
        starts.push((current.0, start));
    }

    starts = remove_dups(starts);

    // convert all numbers
    let mut products: Vec<u64> = Vec::new();
    for tuple in starts {
        let mut number: Vec<char> = Vec::new();
        for val in tuple.1..matrix[tuple.0].len() {
            match matrix[tuple.0][val] {
                '0'..='9' => {
                    number.push(matrix[tuple.0][val]);
                },
                _ => {
                    break;
                },
            }
        }
        products.push(vec_to_number(number));
    }
    // only if there are really 2 products
    return if products.len() == 2 {
        let mut res: u64 = 1;
        for i in products {
            res *= i;
        }
        res
    }else {
        0
    }
}

fn remove_dups(starts: Vec<(usize, usize)>) -> Vec<(usize, usize)> {
    let mut unique: Vec<(usize, usize)> = Vec::new();
    for i in starts {
        if !unique.contains(&i) {
            unique.push(i);
        }
    }
    unique
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