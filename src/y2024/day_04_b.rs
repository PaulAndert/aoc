use std::fs;

pub fn main() {
    let contents = fs::read_to_string("./src/y2024/resources/day_04").expect("Should have been able to read the file");
    let mut cnt: u32 = 0;

    let mut matrix: Vec<Vec<char>> = Vec::new();
    for line in contents.split("\n") {
        let chars: Vec<char> = line.chars().collect();
        if chars.len() > 0 {
            matrix.push(chars);
        }
    }

    for idx in 1..matrix.len()-1 {
        for jdx in 1..matrix[idx].len()-1 {
            if matrix[idx][jdx] == 'A' {
                // top left - bottom right AND top left - bottom left
                if ((matrix[idx - 1][jdx - 1] == 'M' && matrix[idx + 1][jdx + 1] == 'S') ||
                    (matrix[idx - 1][jdx - 1] == 'S' && matrix[idx + 1][jdx + 1] == 'M')) &&
                   ((matrix[idx - 1][jdx + 1] == 'M' && matrix[idx + 1][jdx - 1] == 'S') ||
                    (matrix[idx - 1][jdx + 1] == 'S' && matrix[idx + 1][jdx - 1] == 'M')) {
                        cnt += 1;
                }
            }
        }
    }

    println!("C: {}", cnt);

}