use std::fs;
use std::thread::current;

pub fn main() {
    let contents = fs::read_to_string("./resources/day_10").expect("Should have been able to read the file");
    let mut lines: Vec<&str> = contents.split("\n").collect();

    let mut matrix: Vec<Vec<char>> = Vec::new();

    for line in lines {
        let chars: Vec<char> = line.chars().collect();
        matrix.push(chars);
    }

    // get starting position
    let mut current_pos: (usize, usize) = (0, 0);
    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            if matrix[i][j] == 'S' {
                current_pos = (i, j);
            }
        }
    }

    let mut last_pos: (usize, usize) = current_pos;
    let mut not_found: bool = true;
    // make first step
    // under
    if not_found && current_pos.0 < matrix.len() - 1 && vec!['|', 'L', 'J'].contains(&matrix[current_pos.0 + 1][current_pos.1]) {
        current_pos.0 += 1;
        not_found = false;
    }
    // above
    if not_found && current_pos.0 > 0 && vec!['|', 'F', '7'].contains(&matrix[current_pos.0 - 1][current_pos.1]) {
        current_pos.0 -= 1;
        not_found = false;
    }
    // right
    if not_found && current_pos.1 < matrix[current_pos.0].len() - 1 && vec!['-', 'J', '7'].contains(&matrix[current_pos.0][current_pos.1 + 1]) {
        current_pos.1 += 1;
        not_found = false;
    }
    // right
    if not_found && current_pos.1 > 0 && vec!['-', 'F', 'L'].contains(&matrix[current_pos.0][current_pos.1 - 1]) {
        current_pos.1 -= 1;
        not_found = false;
    }

    let mut steps: u64 = 0;
    loop {
        steps += 1;
        match matrix[current_pos.0][current_pos.1] {
            '|' => {
                if last_pos.0 < current_pos.0 {
                    // came from above
                    last_pos = current_pos;
                    current_pos.0 += 1;
                }else {
                    // came from below
                    last_pos = current_pos;
                    current_pos.0 -= 1;
                }
            },
            '-' => {
                if last_pos.1 < current_pos.1 {
                    // came from left
                    last_pos = current_pos;
                    current_pos.1 += 1;
                }else {
                    // came from right
                    last_pos = current_pos;
                    current_pos.1 -= 1;
                }
            },
            'F' => {
                if last_pos.1 > current_pos.1 {
                    // came from right
                    last_pos = current_pos;
                    current_pos.0 += 1;
                }else if last_pos.0 > current_pos.0 {
                    // came from below
                    last_pos = current_pos;
                    current_pos.1 += 1;
                }else {
                    panic!("Unknown entry direction in {:?}", current_pos);
                }
            },
            'L' => {
                if last_pos.1 > current_pos.1 {
                    // came from right
                    last_pos = current_pos;
                    current_pos.0 -= 1;
                }else if last_pos.0 < current_pos.0 {
                    // came from above
                    last_pos = current_pos;
                    current_pos.1 += 1;
                }else {
                    panic!("Unknown entry direction in {:?}", current_pos);
                }
            },
            '7' => {
                if last_pos.1 < current_pos.1 {
                    // came from left
                    last_pos = current_pos;
                    current_pos.0 += 1;
                }else if last_pos.0 > current_pos.0 {
                    // came from below
                    last_pos = current_pos;
                    current_pos.1 -= 1;
                }else {
                    panic!("Unknown entry direction in {:?}", current_pos);
                }
            },
            'J' => {
                if last_pos.1 < current_pos.1 {
                    // came from left
                    last_pos = current_pos;
                    current_pos.0 -= 1;
                }else if last_pos.0 < current_pos.0 {
                    // came from above
                    last_pos = current_pos;
                    current_pos.1 -= 1;
                }else {
                    panic!("Unknown entry direction in {:?}", current_pos);
                }
            },
            'S' => {
                println!("Steps: {}", steps / 2);
                break;
            },
            _ => panic!("Unknown character in {:?}", current_pos),
        }
    }
}