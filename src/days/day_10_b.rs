use std::fmt::format;
use std::fs;
use std::panic::panic_any;
use std::thread::current;

#[derive(Debug, Clone, PartialEq)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug, Clone)]
struct Line {
    a: Point,
    b: Point,
}

pub fn main() {
    let contents = fs::read_to_string("./resources/day_10").expect("Should have been able to read the file");
    let mut lines: Vec<&str> = contents.split("\n").collect();

    // build matrix
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
                break;
            }
        }
    }

    // get first direction
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


    // create tunnel (all points on the tunnel), tunnel_reduced (all edge points of the tunnel)
    let mut tunnel: Vec<Point> = vec![Point{x: current_pos.0, y: current_pos.1}];
    let mut tunnel_reduced: Vec<Point> = tunnel.clone();

    // choose direction (some inputs need starting direction 0, others direction 1)
    println!("Starts: {:?}", starts);
    current_pos = starts[1];

    // map out the tunnel
    let mut last: char = 'S';
    loop {
        match matrix[current_pos.0][current_pos.1] {
            '|' => {
                if last_pos.0 < current_pos.0 {
                    // came from above
                    last_pos = current_pos;
                    current_pos.0 += 1;
                    matrix[last_pos.0][last_pos.1] = 'v';
                } else {
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
                } else {
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
                } else if last_pos.0 > current_pos.0 {
                    // came from below
                    last_pos = current_pos;
                    current_pos.1 += 1;
                    matrix[last_pos.0][last_pos.1] = '>';
                } else {
                    panic!("Unknown entry direction in {:?}", current_pos);
                }
            },
            'L' => {
                if last_pos.1 > current_pos.1 {
                    // came from right
                    last_pos = current_pos;
                    current_pos.0 -= 1;
                    matrix[last_pos.0][last_pos.1] = '^';
                } else if last_pos.0 < current_pos.0 {
                    // came from above
                    last_pos = current_pos;
                    current_pos.1 += 1;
                    matrix[last_pos.0][last_pos.1] = '>';
                } else {
                    panic!("Unknown entry direction in {:?}", current_pos);
                }
            },
            '7' => {
                if last_pos.1 < current_pos.1 {
                    // came from left
                    last_pos = current_pos;
                    current_pos.0 += 1;
                    matrix[last_pos.0][last_pos.1] = 'v';
                } else if last_pos.0 > current_pos.0 {
                    // came from below
                    last_pos = current_pos;
                    current_pos.1 -= 1;
                    matrix[last_pos.0][last_pos.1] = '<';
                } else {
                    panic!("Unknown entry direction in {:?}", current_pos);
                }
            },
            'J' => {
                if last_pos.1 < current_pos.1 {
                    // came from left
                    last_pos = current_pos;
                    current_pos.0 -= 1;
                    matrix[last_pos.0][last_pos.1] = '^';
                } else if last_pos.0 < current_pos.0 {
                    // came from above
                    last_pos = current_pos;
                    current_pos.1 -= 1;
                    matrix[last_pos.0][last_pos.1] = '<';
                } else {
                    panic!("Unknown entry direction in {:?}", current_pos);
                }
            },
            'S' => {
                last_pos = current_pos;
                break;
            },
            _ => panic!("Unknown character in {:?}", current_pos),
        }
        if matrix[last_pos.0][last_pos.1] != last {
            tunnel_reduced.push(Point{x: last_pos.0, y: last_pos.1});
        }
        last = matrix[last_pos.0][last_pos.1];
        tunnel.push(Point{x: last_pos.0, y: last_pos.1});
    }
    tunnel_reduced.push(Point{x: last_pos.0, y: last_pos.1});

    // check all possible points
    let mut cnt: usize = 0;
    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            let point_to_check = Point{x: i, y: j};
            // if point is NOT on the tunnel and inside the polygon
            if !tunnel.contains(&point_to_check) && inside(tunnel_reduced.clone(), point_to_check) {
                cnt += 1;
            }
        }
    }
    println!("C: {}", cnt);
}

// credit: https://www.geeksforgeeks.org/how-to-check-if-a-given-point-lies-inside-a-polygon/
fn inside(tunnel: Vec<Point>, point: Point) -> bool {
    let n: usize = tunnel.len();
    let mut i: usize = 0;
    let mut cnt: usize = 0;
    let exline = Line{a: point.clone(), b: Point { x: 9999, y: point.y.clone() + 1}};
    loop {
        let side: Line = Line{a: tunnel[i].clone(), b: tunnel[(i + 1) % n].clone() };
        if is_intersect(side.clone(), exline.clone()) {
            if direction(side.a.clone(), point.clone(), side.b.clone()) == 0 {
                return on_line(side, point.clone());
            }
            cnt += 1;
        }
        i = (i + 1) % n;
        if i == 0 { break }
    }
    return cnt % 2 == 1;
}

fn is_intersect(line_1: Line, line_2: Line) -> bool {
    let direction1: i64 = direction(line_1.a.clone(), line_1.b.clone(), line_2.a.clone());
    let direction2: i64 = direction(line_1.a.clone(), line_1.b.clone(), line_2.b.clone());
    let direction3: i64 = direction(line_2.a.clone(), line_2.b.clone(), line_1.a.clone());
    let direction4: i64 = direction(line_2.a.clone(), line_2.b.clone(), line_1.b.clone());

    // When intersecting
    if direction1 != direction2 && direction3 != direction4 {
        return true;
    }

    // When p2 of line2 are on the line1
    if direction1 == 0 && on_line(line_1.clone(), line_2.a.clone()) {
        return true;
    }

    // When p1 of line2 are on the line1
    if direction2 == 0 && on_line(line_1.clone(), line_2.b.clone()) {
        return true;
    }

    // When p2 of line1 are on the line2
    if direction3 == 0 && on_line(line_2.clone(), line_1.a.clone()){
        return true;
    }

    // When p1 of line1 are on the line2
    if direction4 == 0 && on_line(line_2.clone(), line_1.b.clone()) {
        return true;
    }
    false
}

fn direction(a: Point, b: Point, c: Point) -> i64 {
    let val: i64 = (b.y as i64 - a.y as i64) * (c.x as i64 - b.x as i64) - (b.x as i64 - a.x as i64) * (c.y as i64 - b.y as i64);
    // Collinear
    return if val == 0 { 0 }
    // Anti - clockwise direction
    else if val < 0 { 2 }
    // Clockwise direction
    else { 1 }
}

fn on_line(line: Line, point: Point) -> bool {
    // Check whether p is on the line or not
    if point.x <= usize::max(line.a.x, line.b.x) && point.x >= usize::min(line.a.x, line.b.x) &&
        (point.y <= usize::max(line.a.y, line.b.y) && point.y >= usize::min(line.a.y, line.b.y)) {
        return true;
    }
    false
}