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

    // expand rows
    let mut rows: Vec<usize> = Vec::new();
    for i in 0..matrix.len() {
        if matrix[i].iter().sum::<u64>() == 0 {
            rows.push(i);
        }
    }

    let mut indexes: Vec<(usize, usize)> = Vec::new();
    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            match matrix[i][j] {
                0 => {},
                _ => {
                    indexes.push((j, i));
                }
            }
        }
    }

    // calculate sum
    let mut sum: usize = 0;
    for a in 0..indexes.len() {
        for b in (a+1)..indexes.len() {
            if indexes[a] == indexes[b] { continue }
            sum += get_path(rows.clone(), columns.clone(), indexes[a], indexes[b]);
        }
    }
    println!("Sum: {}", sum);
}

fn get_path(rows: Vec<usize>, columns: Vec<usize>, a: (usize, usize), b: (usize, usize)) -> usize {
    // a -> b
    let scale: i64 = 1_000_000;
    let mut x: i64 = (a.0 as i64 - b.0 as i64).abs();
    let mut y: i64 = (a.1 as i64 - b.1 as i64).abs();
    for column in columns {
        if a.0 < column && b.0 > column || a.0 > column && b.0 < column {
            x += scale - 1;
        }
    }
    for row in rows {
        if a.1 < row && b.1 > row || a.1 > row && b.1 < row {
            y += scale - 1;
        }
    }
    return (x + y) as usize;
}

fn print_m(matrix: Vec<Vec<u64>>) {
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