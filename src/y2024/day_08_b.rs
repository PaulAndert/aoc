use std::fs;

pub fn main() {
    let contents = fs::read_to_string("./src/y2024/resources/day_08").expect("Should have been able to read the file");
    let mut cnt: u64 = 0;

    let mut antennas: Vec<char> = Vec::new();
    let mut matrix: Vec<Vec<char>> = Vec::new();
    for line in contents.split("\n") {
        let chars: Vec<char> = line.chars().collect();
        if chars.len() <= 1 {
            continue;
        }
        matrix.push(chars.clone());
        for c in chars {
            if c != '.' && !antennas.contains(&c) {
                antennas.push(c);
            }
        }
    }

    let mut antinode_matrix: Vec<Vec<char>> = matrix.clone();
    for antenna in antennas {
        let mut antenna_positions: Vec<(usize, usize)> = Vec::new();
        for idx_y in 0..matrix.len() {
            for idx_x in 0..matrix[idx_y].len() {
                if matrix[idx_y][idx_x] == antenna {
                    antenna_positions.push((idx_y, idx_x));
                }
            }
        }

        // get all antenna_positions permutations
        for perm in get_all_permutations(antenna_positions.len()) {
            // println!("{} - {:?}", antenna, perm);
            // println!("{:?}, {:?}", antenna_positions[perm.0], antenna_positions[perm.1]);
            let move_y: i32 = antenna_positions[perm.1].0 as i32 - antenna_positions[perm.0].0 as i32;
            let move_x: i32 = antenna_positions[perm.1].1 as i32 - antenna_positions[perm.0].1 as i32;
            // println!("{} , {}", move_y, move_x);

            let first_new_y: i32 = antenna_positions[perm.1].0 as i32 + move_y;
            let first_new_x: i32 = antenna_positions[perm.1].1 as i32 + move_x;
            if first_new_y >= 0 && first_new_y < antinode_matrix.len() as i32 && first_new_x >= 0 && first_new_x < antinode_matrix[first_new_y as usize].len() as i32 {
                antinode_matrix[first_new_y as usize][first_new_x as usize] = '#';
            }

            let second_new_y: i32 = antenna_positions[perm.0].0 as i32 - move_y;
            let second_new_x: i32 = antenna_positions[perm.0].1 as i32 - move_x;
            if second_new_y >= 0 && second_new_y < antinode_matrix.len() as i32 && second_new_x >= 0 && second_new_x < antinode_matrix[second_new_y as usize].len() as i32 {
                antinode_matrix[second_new_y as usize][second_new_x as usize] = '#';
            }

            // for line in antinode_matrix.clone() {
            //     // println!("{:?}", line);
            //     for c in line {
            //         print!("{} ", c);
            //     }
            //     println!("");
            // }
        }
    }


    for line in antinode_matrix.clone() {
        // println!("{:?}", line);
        for c in line {
            // print!("{} ", c);
            if c == '#' {
                cnt += 1;
            }
        }
        // println!("");
    }

    println!("C: {}", cnt);
}

fn get_all_permutations(size: usize) -> Vec<(usize, usize)> {
    let mut permutations: Vec<(usize, usize)> = Vec::new();
    let mut options: Vec<usize> = Vec::new();
    for i in 0..size {
        options.push(i);
    }

    for _op in options.clone() {
        let first = options[0];
        options.remove(0);
        for second in options.clone() {
            permutations.push((first, second));
        }
    }

    return permutations;
}