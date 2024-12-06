use std::fs;

pub fn main() {
    let contents = fs::read_to_string("./src/y2024/resources/day_06").expect("Should have been able to read the file");
    let mut cnt: u32 = 0;

    let mut matrix: Vec<Vec<char>> = Vec::new();
    let mut guard_x: usize = 0;
    let mut guard_y: usize = 0;

    for line in contents.split("\n") {
        let chars: Vec<char> = line.chars().collect();
        if chars.contains(&'^') {
            guard_y = matrix.len();
            guard_x = chars.iter().position(|&a| a == '^').unwrap();
        }
        if chars.len() > 0 {
            matrix.push(chars);
        }
    }

    let mut direction: char = 'U'; // _U_p, _D_own, _R_ight, _L_eft
    let mut in_map: bool = true;
    while in_map {
        match direction {
            'U' => {
                if guard_y as i32 - 1 < 0 {
                    in_map = false;
                    break;
                }
                if matrix[guard_y - 1][guard_x] == '#' {
                    direction = 'R';
                }else {
                    guard_y -= 1;
                    if matrix[guard_y][guard_x] != 'X' {
                        matrix[guard_y][guard_x] = 'X';
                        cnt += 1
                    }
                }
            },
            'R' => {
                if guard_x + 1 >= matrix[guard_y].len() {
                    in_map = false;
                    break;
                }
                if matrix[guard_y][guard_x + 1] == '#' {
                    direction = 'D';
                }else {
                    guard_x += 1;
                    if matrix[guard_y][guard_x] != 'X' {
                        matrix[guard_y][guard_x] = 'X';
                        cnt += 1
                    }
                }
            },
            'D' => {
                if guard_y + 1 >= matrix.len() {
                    in_map = false;
                    break;
                }
                if matrix[guard_y + 1][guard_x] == '#' {
                    direction = 'L';
                }else {
                    guard_y += 1;
                    if matrix[guard_y][guard_x] != 'X' {
                        matrix[guard_y][guard_x] = 'X';
                        cnt += 1
                    }
                }
            },
            'L' => {
                if guard_x as i32 - 1 < 0 {
                    in_map = false;
                    break;
                }
                if matrix[guard_y][guard_x - 1] == '#' {
                    direction = 'U';
                }else {
                    guard_x -= 1;
                    if matrix[guard_y][guard_x] != 'X' {
                        matrix[guard_y][guard_x] = 'X';
                        cnt += 1
                    }
                }
            },
            _ => { println!("How the fuck did you get here?"); }
        }

    }
    
    println!("C: {}", cnt);
}