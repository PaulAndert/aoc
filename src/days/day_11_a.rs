use std::arch::x86_64::_mm256_undefined_si256;
use std::fmt::format;
use std::fs;

pub fn main() {
    let contents = fs::read_to_string("./resources/day_11").expect("Should have been able to read the file");
    let mut lines: Vec<&str> = contents.split("\n").collect();

    // build matrix
    let mut matrix: Vec<Vec<u64>> = Vec::new();
    let mut index: u64 = 0;
    for line in lines {
        let chars: Vec<char> = line.chars().collect();
        let mut row: Vec<u64> = Vec::new();
        for c in chars {
            match c {
                '.' => row.push(0),
                '#' => {
                    index += 1;
                    row.push(index);
                },
                _ => row.push(0),
            }
        }
        matrix.push(row);
    }

    // print_m(matrix.clone());

    // expand columns
    let mut columns: Vec<usize> = Vec::new();
    for j in 0..matrix[0].len() {
        let mut sum: u8 = 0;
        for i in 0..matrix.len() {
            if matrix[i][j] != 0 {
                sum += 1;
            }
        }
        if sum == 0 {
            columns.push(j);
        }
    }
    for i in 0..matrix.len() {
        let mut new_row = Vec::new();
        for j in 0..matrix[i].len() {
            if columns.contains(&j) {
                new_row.push(0);
            }
            new_row.push(matrix[i][j]);
        }
        matrix.splice(i..i+1, vec![new_row]);
    }

    // print_m(matrix.clone());

    // expand rows
    let mut rows: Vec<usize> = Vec::new();
    for i in 0..matrix.len() {
        if matrix[i].iter().sum::<u64>() == 0 {
            rows.push(i);
        }
    }
    let mut matrix_exp: Vec<Vec<u64>> = Vec::new();
    for i in 0..matrix.len() {
        if rows.contains(&i) {
            matrix_exp.push(matrix[i].clone());
        }
        matrix_exp.push(matrix[i].clone());
    }

    // print_m(matrix_exp.clone());

    let mut indexes: Vec<(usize, usize)> = Vec::new();
    for i in 0..matrix_exp.len() {
        for j in 0..matrix_exp[i].len() {
            match matrix_exp[i][j] {
                0 => {},
                _ => {
                    indexes.push((i, j));
                }
            }
        }
    }

    let mut sum: usize = 0;
    // get shortest paths
    for a in 0..indexes.len() {
        for b in (a+1)..indexes.len() {
            if indexes[a] == indexes[b] { continue }
            sum += get_path(indexes[a], indexes[b]);
        }
    }
    println!("Sum: {}", sum);
}

fn get_path(a: (usize, usize), b: (usize, usize)) -> usize {
    // a -> b
    return ( (a.0 as i64 - b.0 as i64).abs() + (a.1 as i64 - b.1 as i64).abs() ) as usize;
}

fn print_m(matrix: Vec<Vec<u64>>) {
    let mut index: usize = 0;
    println!("\nMatrix {}x{}", matrix.len(), matrix[0].len());
    for i in 0..matrix.len() {
        let mut line: String = if i < 10 {
            format!("0{}: ", i)
        }else {
            format!("{}: ", i)
        };
        for j in 0..matrix[i].len() {
            match matrix[i][j] {
                0 => line = format!("{}.", line),
                _ => line = format!("{}{}", line, matrix[i][j]),
            }
        }
        println!("{}", line);
    }
}