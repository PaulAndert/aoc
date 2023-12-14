use std::fmt::format;
use std::fs;
use std::panic::panic_any;
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

    let mut starts: Vec<(usize, usize)> = Vec::new();
    let mut last_pos: (usize, usize) = current_pos;
    // make first step
    // under
    if current_pos.0 < matrix.len() - 1 && vec!['|', 'L', 'J'].contains(&matrix[current_pos.0 + 1][current_pos.1]) {
        starts.push((current_pos.0 + 1, current_pos.1));
    }
    // above
    if current_pos.0 > 0 && vec!['|', 'F', '7'].contains(&matrix[current_pos.0 - 1][current_pos.1]) {
        starts.push((current_pos.0 - 1, current_pos.1));
    }
    // right
    if current_pos.1 < matrix[current_pos.0].len() - 1 && vec!['-', 'J', '7'].contains(&matrix[current_pos.0][current_pos.1 + 1]) {
        starts.push((current_pos.0, current_pos.1 + 1));
    }
    // right
    if current_pos.1 > 0 && vec!['-', 'F', 'L'].contains(&matrix[current_pos.0][current_pos.1 - 1]) {
        starts.push((current_pos.0, current_pos.1 - 1));
    }

    // choose direction
    println!("{:?}", starts);
    current_pos = starts[1];

    // find tunnel
    let curr_pos_save: (usize, usize) = current_pos.clone();
    loop {
        match matrix[current_pos.0][current_pos.1] {
            '|' => {
                if last_pos.0 < current_pos.0 {
                    // came from above
                    last_pos = current_pos;
                    current_pos.0 += 1;
                    matrix[last_pos.0][last_pos.1] = 'v';
                }else {
                    // came from below
                    last_pos = current_pos;
                    current_pos.0 -= 1;
                    matrix[last_pos.0][last_pos.1] = '^';
                }
            },
            '-' => {
                if last_pos.1 < current_pos.1 {
                    // came from left
                    last_pos = current_pos;
                    current_pos.1 += 1;
                    matrix[last_pos.0][last_pos.1] = '>';
                }else {
                    // came from right
                    last_pos = current_pos;
                    current_pos.1 -= 1;
                    matrix[last_pos.0][last_pos.1] = '<';
                }
            },
            'F' => {
                if last_pos.1 > current_pos.1 {
                    // came from right
                    last_pos = current_pos;
                    current_pos.0 += 1;
                    matrix[last_pos.0][last_pos.1] = 'v';
                }else if last_pos.0 > current_pos.0 {
                    // came from below
                    last_pos = current_pos;
                    current_pos.1 += 1;
                    matrix[last_pos.0][last_pos.1] = '>';
                }else {
                    panic!("Unknown entry direction in {:?}", current_pos);
                }
            },
            'L' => {
                if last_pos.1 > current_pos.1 {
                    // came from right
                    last_pos = current_pos;
                    current_pos.0 -= 1;
                    matrix[last_pos.0][last_pos.1] = '^';
                }else if last_pos.0 < current_pos.0 {
                    // came from above
                    last_pos = current_pos;
                    current_pos.1 += 1;
                    matrix[last_pos.0][last_pos.1] = '>';
                }else {
                    panic!("Unknown entry direction in {:?}", current_pos);
                }
            },
            '7' => {
                if last_pos.1 < current_pos.1 {
                    // came from left
                    last_pos = current_pos;
                    current_pos.0 += 1;
                    matrix[last_pos.0][last_pos.1] = 'v';
                }else if last_pos.0 > current_pos.0 {
                    // came from below
                    last_pos = current_pos;
                    current_pos.1 -= 1;
                    matrix[last_pos.0][last_pos.1] = '<';
                }else {
                    panic!("Unknown entry direction in {:?}", current_pos);
                }
            },
            'J' => {
                if last_pos.1 < current_pos.1 {
                    // came from left
                    last_pos = current_pos;
                    current_pos.0 -= 1;
                    matrix[last_pos.0][last_pos.1] = '^';
                }else if last_pos.0 < current_pos.0 {
                    // came from above
                    last_pos = current_pos;
                    current_pos.1 -= 1;
                    matrix[last_pos.0][last_pos.1] = '<';
                }else {
                    panic!("Unknown entry direction in {:?}", current_pos);
                }
            },
            'S' => {
                last_pos = current_pos;
                break;
            },
            _ => panic!("Unknown character in {:?}", current_pos),
        }
    }

    // remove everything except tunnel
    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            match matrix[i][j] {
                'v' | '^' | '<' | '>' | 'S' => {},
                _ => matrix[i][j] = ' ',
            }
        }
    }

    // search enclosed I fields
    current_pos = curr_pos_save;
    let mut pipe_symbols: Vec<char> = vec!['v', '^', '>', '<', 'S'];
    loop {
        match matrix[current_pos.0][current_pos.1] {
            'v' => {
                // circle right -> matrix left
                let mut new_i: Vec<(usize, usize)> = Vec::new();
                for n_j in (0..current_pos.1).rev() {
                    if !pipe_symbols.contains(&matrix[current_pos.0][n_j]) {
                        new_i.push((current_pos.0, n_j));
                    }else {
                        for i in &new_i { matrix[i.0][i.1] = 'I'; }
                        break;
                    }
                }
                last_pos = current_pos;
                current_pos.0 += 1;
            },
            '^' => {
                // circle right -> matrix right
                let mut new_i: Vec<(usize, usize)> = Vec::new();
                for n_j in (current_pos.1 + 1)..(matrix[current_pos.0].len() - 1) {
                    if !pipe_symbols.contains(&matrix[current_pos.0][n_j]) {
                        new_i.push((current_pos.0, n_j));
                    }else {
                        for i in &new_i { matrix[i.0][i.1] = 'I'; }
                        break;
                    }
                }

                last_pos = current_pos;
                current_pos.0 -= 1;
            },
            '>' => {
                // circle right -> matrix down
                let mut new_i: Vec<(usize, usize)> = Vec::new();
                for n_i in (current_pos.0 + 1)..(matrix.len() - 1) {
                    if !pipe_symbols.contains(&matrix[n_i][current_pos.1]) {
                        new_i.push((n_i, current_pos.1));
                    }else {
                        for i in &new_i { matrix[i.0][i.1] = 'I'; }
                        break;
                    }
                }
                last_pos = current_pos;
                current_pos.1 += 1;
            },
            '<' => {
                // circle right -> matrix up
                let mut new_i: Vec<(usize, usize)> = Vec::new();
                for n_i in (0..current_pos.0).rev() {
                    if !pipe_symbols.contains(&matrix[n_i][current_pos.1]) {
                        new_i.push((n_i, current_pos.1));
                    }else {
                        for i in &new_i { matrix[i.0][i.1] = 'I'; }
                        break;
                    }
                }
                last_pos = current_pos;
                current_pos.1 -= 1;
            },
            'S' => break,
            _ => {},
        }
    }

    // search all neighbors of every I
    pipe_symbols.push('I');
    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            if matrix[i][j] == 'I' {
                let mut new_i: Vec<(usize, usize)> = vec![(i, j)];
                while new_i.len() > 0 {
                    println!("{:?}", new_i);
                    let curr: (usize, usize) = new_i[0];
                    // down
                    if curr.0 < matrix.len() && !pipe_symbols.contains(&matrix[curr.0 + 1][curr.1]) {
                        matrix[curr.0 + 1][curr.1] = 'I';
                        new_i.push((curr.0 + 1, curr.1));
                    }
                    // up
                    if curr.0 > 0 && !pipe_symbols.contains(&matrix[curr.0 - 1][curr.1]) {
                        matrix[curr.0 - 1][curr.1] = 'I';
                        new_i.push((curr.0 - 1, curr.1));
                    }
                    // right
                    if curr.1 < matrix[curr.0].len() && !pipe_symbols.contains(&matrix[curr.0][curr.1 + 1]) {
                        matrix[curr.0][curr.1 + 1] = 'I';
                        new_i.push((curr.0, curr.1 + 1));
                    }
                    // left
                    if curr.1 > 0 && !pipe_symbols.contains(&matrix[curr.0][curr.1 - 1]) {
                        matrix[curr.0][curr.1 - 1] = 'I';
                        new_i.push((curr.0, curr.1 - 1));
                    }
                    new_i.remove(0);
                }
            }
        }
    }

    print_matrix(matrix.clone());

    // count inside areas
    let mut cnt: usize = 0;
    for arr in &matrix {
        for c in arr {
            if *c == 'I' { cnt += 1 }
        }
    }
    println!("C: {}", cnt);


    let mut big: Vec<Vec<char>> = Vec::new();
    for i in 0..matrix.len() {
        let mut line : Vec<char> = Vec::new();
        for j in 0..matrix[i].len() {
            match matrix[i][j] {
                '>' => {
                    line.push('>');
                    line.push('>');
                },
                '<' => {
                    line.push('<');
                    line.push('<');
                },
                '^' => {
                    line.push(' ');
                    line.push('^');
                },
                'v' => {
                    line.push('v');
                    line.push(' ');
                },
                'S' => {
                    line.push(' ');
                    line.push('S');
                },
                ' ' => {
                    line.push(' ');
                    line.push(' ');
                },
                'I' => {
                    line.push('I');
                    line.push('I');
                },
                _ => panic!("Error: unknown char: {}", matrix[i][j]),
            }
        }
        big.push(line);
    }
    // fill gaps
    for i in 0..big.len() {
        for j in 0..big[i].len() {
            match big[i][j] {
                '>' => {
                    if j < big[i].len() - 1 && big[i][j + 1] == ' ' {
                        big[i][j + 1] = '>';
                    }
                },
                '<' => {
                    if j > 0 && big[i][j - 1] == ' ' {
                        big[i][j - 1] = '<';
                    }
                },
                _ => {},
            }
        }
    }
    print_matrix(big);
}

fn print_matrix(matrix: Vec<Vec<char>>) {
    for i in 0..matrix.len() {
        let mut line: String = format!("{}: ", i);
        for c in &matrix[i] {
            line = if *c == 'I' {
                format!("{}\x1b[93m{}\x1b[0m", line, c)
            }else if *c == 'Z' {
                format!("{}\x1b[91m{}\x1b[0m", line, c)
            }else if *c == ' ' {
                format!("{}.", line)
            }else {
                format!("{}{}", line, c)
            }
        }
        println!("{}", line);
    }
}