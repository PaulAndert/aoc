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

    // horizontal
    for idx in 0..matrix.len() {
        cnt += count_xmas(matrix[idx].clone());
    }

    // vertical
    for idx in 0..matrix[0].len() {
        let mut line: Vec<char> = Vec::new();
        for jdx in 0..matrix.len() {
            line.push(matrix[jdx][idx]);
        }
        cnt += count_xmas(line);
    }

    // diagonally right
    // x axis
    for idx in 0..matrix[0].len() {
        
        let mut line: Vec<char> = Vec::new();
        let mut loc_offset: usize = 0;

        while loc_offset < matrix.len() && (idx + loc_offset) < matrix[loc_offset].len() {
            line.push(matrix[loc_offset][idx + loc_offset]);
            loc_offset += 1;
        }
        cnt += count_xmas(line);
    }
    
    // y axis
    for idx in 1..matrix.len() {
        
        let mut line: Vec<char> = Vec::new();
        let mut loc_offset: usize = 0;
        
        while (idx + loc_offset) < matrix.len() && loc_offset < matrix[loc_offset].len() {
            line.push(matrix[idx + loc_offset][loc_offset]);
            loc_offset += 1;
        }
        cnt += count_xmas(line);
    }
    
    // diagonally left
    // x axis
    for idx in 0..matrix[0].len() {
        
        let mut line: Vec<char> = Vec::new();
        let mut loc_offset: usize = 0;
        
        while loc_offset < matrix.len() && (idx as i32 - loc_offset as i32) >= 0 {
            line.push(matrix[loc_offset][idx - loc_offset]);
            loc_offset += 1;
        }
        cnt += count_xmas(line);
    }
    
    // y axis
    for idx in 1..matrix.len() {
        
        let mut line: Vec<char> = Vec::new();
        let mut loc_offset: usize = 0;
        
        let end = matrix[loc_offset].len() - 1;
        while idx + loc_offset < matrix.len() && (end as i32 - loc_offset as i32) > 0 {
            line.push(matrix[idx + loc_offset][end - loc_offset]);
            loc_offset += 1;
        }
        cnt += count_xmas(line);
    }

    println!("C: {}", cnt);

}

fn count_xmas(line: Vec<char>) -> u32 {
    let mut cnt = 0;

    let mut idx: usize = 0;
    while idx < line.len() {
        if idx + 3 < line.len() && line[idx] == 'X' && line[idx + 1] == 'M' && line[idx + 2] == 'A' && line[idx + 3] == 'S' {
            cnt += 1;
        }
        if idx + 3 < line.len() && line[idx] == 'S' && line[idx + 1] == 'A' && line[idx + 2] == 'M' && line[idx + 3] == 'X' {
            cnt += 1
        }
        idx += 1
    }

    return cnt;
}
