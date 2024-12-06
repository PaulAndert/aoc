use std::fs;

pub fn main() {
    let contents = fs::read_to_string("./src/y2024/resources/day_06").expect("Should have been able to read the file");
    let mut cnt: u32 = 0;

    let mut matrix: Vec<Vec<Vec<i32>>> = Vec::new();
    let mut guard_x: usize = 0;
    let mut guard_y: usize = 0;

    for line in contents.split("\n") {
        let chars: Vec<char> = line.chars().collect();
        let nums: Vec<Vec<i32>> = chars.iter().map(|c| {
            match c { 
                '#' => vec![-1],
                '^' => vec![1],
                _ => Vec::new(),
            }
        }).collect();
        if chars.contains(&'^') {
            guard_y = matrix.len();
            guard_x = chars.iter().position(|&a| a == '^').unwrap();
        }
        if chars.len() > 0 {
            matrix.push(nums);
        }
    }

    for idx_y in 0..matrix.len() {
        for idx_x in 0..matrix[idx_y].len() {
            let mut loc_matrix: Vec<Vec<Vec<i32>>> = matrix.clone();
            if loc_matrix[idx_y][idx_x].contains(&-1) {
                continue;
            }
            loc_matrix[idx_y][idx_x] = vec![-1];

            if test_loc_matrix(guard_y, guard_x, loc_matrix) {
                cnt += 1;
            }
        }
    }

    println!("C: {}", cnt);
}

fn check_loop(loc_guard_y: usize, loc_guard_x: usize, loc_matrix: Vec<Vec<Vec<i32>>>, last_loop_number: Vec<i32>) -> (bool, Vec<Vec<Vec<i32>>>) {
    if loc_matrix[loc_guard_y][loc_guard_x].len() > 0 && last_loop_number.len() > 0 && last_loop_number.iter().any(|&n| loc_matrix[loc_guard_y][loc_guard_x].contains(&(n + 1)) ) {
        return (true, loc_matrix);
    }
    return (false, loc_matrix);
}

fn test_loc_matrix(guard_y: usize, guard_x: usize, mut loc_matrix: Vec<Vec<Vec<i32>>>) -> bool {
    let mut loc_guard_x: usize = guard_x;
    let mut loc_guard_y: usize = guard_y;

    let mut direction: char = 'U'; // _U_p, _D_own, _R_ight, _L_eft
    let mut index: i32 = 2;
    let mut last_loop_number: Vec<i32> = Vec::new();
    let mut loop_found: bool = false;

    while !loop_found{
        match direction {
            'U' => {
                if loc_guard_y as i32 - 1 < 0 {
                    return false;
                }
                if loc_matrix[loc_guard_y - 1][loc_guard_x].contains(&-1) {
                    direction = 'R';
                }else {
                    loc_guard_y -= 1;
                    (loop_found, loc_matrix) = check_loop(loc_guard_y, loc_guard_x, loc_matrix, last_loop_number);
                    if loop_found {
                        return true;
                    }
                    last_loop_number = loc_matrix[loc_guard_y][loc_guard_x].clone();
                    loc_matrix[loc_guard_y][loc_guard_x].push(index);
                    index += 1;
                }
            },
            'R' => {
                if loc_guard_x + 1 >= loc_matrix[loc_guard_y].len() {
                    return false;
                }
                if loc_matrix[loc_guard_y][loc_guard_x + 1].contains(&-1) {
                    direction = 'D';
                }else {
                    loc_guard_x += 1;
                    (loop_found, loc_matrix) = check_loop(loc_guard_y, loc_guard_x, loc_matrix, last_loop_number);
                    if loop_found {
                        return true;
                    }
                    last_loop_number = loc_matrix[loc_guard_y][loc_guard_x].clone();
                    loc_matrix[loc_guard_y][loc_guard_x].push(index);
                    index += 1;
                }
            },
            'D' => {
                if loc_guard_y + 1 >= loc_matrix.len() {
                    return false;
                }
                if loc_matrix[loc_guard_y + 1][loc_guard_x].contains(&-1) {
                    direction = 'L';
                }else {
                    loc_guard_y += 1;
                    (loop_found, loc_matrix) = check_loop(loc_guard_y, loc_guard_x, loc_matrix, last_loop_number);
                    if loop_found {
                        return true;
                    }
                    last_loop_number = loc_matrix[loc_guard_y][loc_guard_x].clone();
                    loc_matrix[loc_guard_y][loc_guard_x].push(index);
                    index += 1;
                }
            },
            'L' => {
                if loc_guard_x as i32 - 1 < 0 {
                    return false;
                }
                if loc_matrix[loc_guard_y][loc_guard_x - 1].contains(&-1) {
                    direction = 'U';
                }else {
                    loc_guard_x -= 1;
                    (loop_found, loc_matrix) = check_loop(loc_guard_y, loc_guard_x, loc_matrix, last_loop_number);
                    if loop_found {
                        return true;
                    }
                    last_loop_number = loc_matrix[loc_guard_y][loc_guard_x].clone();
                    loc_matrix[loc_guard_y][loc_guard_x].push(index);
                    index += 1;
                }
            },
            _ => { println!("How the fuck did you get here?"); }
        }

    }
    return loop_found;
}

fn print_m(loc_matrix: Vec<Vec<Vec<i32>>>) {
    for l in loc_matrix {
        for e in l {
            if e.len() == 0 {
                print!("0\t");
            }else if e.len() == 1 {
                print!("{}\t", e[0]);
            }else {
                let mut ret: String = String::new();
                for ee in e {
                    ret = format!("{}|{}", ret, ee);
                }
                print!("{}\t", ret);
            }
        }
        println!("");
    }
}