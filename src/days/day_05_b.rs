use std::fs;

pub fn main() {
    let contents = fs::read_to_string("./resources/day_05").expect("Should have been able to read the file");
    let mut lines: Vec<&str> = contents.split("\n").collect();

    let seed_parts: Vec<&str> = lines[0].split(": ").collect();
    lines.remove(0);
    let seeds: Vec<i64> = seed_parts[1].split(" ").map(|s| str_to_number(s)).collect();

    // build cube: stages :: lines :: 3 numbers
    let mut cube: Vec<Vec<Vec<i64>>> = vec![Vec::new()];
    let mut stage: usize = 0;
    for line in lines {
        match line.chars().next() {
            Some('0'..='9') => {
                let numbers: Vec<i64> = line.split(" ").map(|s| str_to_number(s)).collect();
                cube[stage].push(numbers);
            },
            _ => {
                if line != "" {
                    stage += 1;
                    cube.push(Vec::new());
                }
            },
        }
    }

    // calculate the boundaries of the number blocks
    let mut cube_bounds: Vec<Vec<i64>> = Vec::new();
    for index in 1..cube.len() {
        let mut bounds: Vec<i64> = Vec::new();
        for arr in &cube[index] {
            bounds.push(arr[1]);
            bounds.push(arr[1] + arr[2] - 1);
        }
        bounds.sort();
        let mut new_bounds: Vec<i64> = Vec::new();
        let mut lowest: i64 = bounds[0];
        for i in (1..bounds.len() - 1).step_by(2) {
            if bounds[i] + 1 != bounds[i + 1] {
                new_bounds.push(lowest);
                new_bounds.push(bounds[i]);
                lowest = bounds[i + 1];
            }
        }
        new_bounds.push(lowest);
        new_bounds.push(bounds[bounds.len() - 1]);
        cube_bounds.push(new_bounds);
    }

    cube.remove(0);
    let mut results: Vec<i64> = Vec::new();

    // calculate all possible boundaries
    for i in (0..seeds.len()).step_by(2) {
        results.push(get_step_bounds(cube.clone(), seeds[i], seeds[i] + seeds[i+1] - 1, 0));
    }
    println!("Res: {}", results.iter().min().unwrap());
}


fn get_step_bounds(cube: Vec<Vec<Vec<i64>>>, start: i64, end: i64, depth: usize) -> i64 {
    let max_depth: usize = cube.len() - 1;
    return if depth < cube.len() {
        let mut b_cnt: usize = 0;
        let mut ret: i64 = i64::MAX;

        // search a match in all lines per gardening step
        for line in &cube[depth] {
            let local_start: i64 = line[1];
            let local_end: i64 = line[1] + line[2] - 1;
            let local_diff: i64 = line[1] - line[0];

            if start >= local_start && end <= local_end {
                // if start and end are fully inside a match
                return if depth == max_depth {
                    start - local_diff
                }else {
                    get_step_bounds(cube.clone(), start - local_diff, end - local_diff, depth + 1)
                }
            } else if start > local_end || end < local_start {
                // if start and end are fully outside a match
                b_cnt += 1;
                continue;
            } else {
                // one part is inside other is outside
                if start >= local_start && start <= local_end {
                    // start is inside
                    let mut a = get_step_bounds(cube.clone(), start - local_diff, local_end - local_diff, depth + 1);

                    // for all lines check if ...end is somewhere else
                    let mut c1_cnt: usize = 0;
                    for other_line in &cube[depth] {
                        if end >= other_line[1] && end <= other_line[1] + other_line[2] - 1 {
                            let b = get_step_bounds(cube.clone(), other_line[1] - (other_line[1] - other_line[0]), end - (other_line[1] - other_line[0]), depth + 1);
                            if b < a {
                                a = b
                            }
                            break;
                        } else {
                            c1_cnt += 1;
                        }
                    }
                    // if no other match was found
                    if c1_cnt == cube[depth].len() {
                        if depth == max_depth {
                            return get_min(vec![ret, local_end + 1]);
                        } else {
                            return get_step_bounds(cube.clone(), local_end + 1, end, depth + 1);
                        }
                    }
                    // if its the last step
                    if depth == max_depth {
                        return get_min(vec![ret, start - local_diff, local_start - local_diff, a]);
                    }
                    return a;
                } else {
                    // end is inside
                    let mut a = get_step_bounds(cube.clone(), local_start - local_diff, end - local_diff, depth + 1);

                    // for all lines check if start... is somewhere else
                    let mut c2_cnt: usize = 0;
                    for other_line in &cube[depth] {
                        if start >= other_line[1] && start <= other_line[1] + other_line[2] - 1 {
                            let b = get_step_bounds(cube.clone(), start - (other_line[1] - other_line[0]), (local_start - 1) - (other_line[1] - other_line[0]), depth + 1);
                            if b < a {
                                a = b
                            }
                            break;
                        } else {
                            c2_cnt += 1;
                        }
                    }
                    // if no other match was found
                    if c2_cnt == cube[depth].len() {
                        if depth == max_depth {
                            return get_min(vec![ret, start]);
                        } else {
                            return get_step_bounds(cube.clone(), start, local_start - 1, depth + 1);
                        }
                    }
                    // if its the last step
                    if depth == max_depth {
                        return get_min(vec![ret, start - local_diff, local_start - local_diff]);
                    }
                }
            }
        }
        // if no match was found
        if b_cnt == cube[depth].len() {
            if depth == max_depth {
                ret = get_min(vec![ret, start]);
            } else {
                return get_step_bounds(cube.clone(), start, end, depth + 1);
            }
        }
        ret
    } else {
        start
    }
}

fn get_min(ve: Vec<i64>) -> i64 {
    return *ve.iter().min().unwrap();
}

fn str_to_number(string_number: &str) -> i64 {
    let chars: Vec<char> = string_number.chars().collect();
    let mut cnt: u32 = 0;
    let mut number: i64 = 0;
    for n in (0..chars.len()).rev() {
        number += (chars[n] as i64 - '0' as i64) * i64::pow(10, cnt);
        cnt += 1;
    }
    return number;
}