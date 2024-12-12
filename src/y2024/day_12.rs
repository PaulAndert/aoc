use std::fs;

pub fn main_a() {
    let contents = fs::read_to_string("./src/y2024/resources/day_12").expect("Should have been able to read the file");
    let mut cnt = 0;

    let mut garden: Vec<Vec<char>> = Vec::new();
    let mut region: Vec<Vec<u32>> = Vec::new();
    for line in contents.split("\n") {
        let chars: Vec<char> = line.chars().collect();
        if chars.len() <= 1 {
            continue;
        }
        region.push(vec![0; chars.len()]);
        garden.push(chars);
    }

    let mut region_cnt: u32 = 1;
    for idx_y in 0..garden.len() {
        for idx_x in 0..garden[idx_y].len() {
            if region[idx_y][idx_x] == 0 {
                (garden, region) = breadth_search(idx_y, idx_x, region_cnt, garden, region);
                region_cnt += 1;
            }
        }
    }

    let mut perimeter: Vec<u32> = vec![0; region_cnt as usize];
    let mut area: Vec<u32> = vec![0; region_cnt as usize];
    for idx_y in 0..garden.len() {
        for idx_x in 0..garden[idx_y].len() {
            let region_number = region[idx_y][idx_x];
            let mut fences: u32 = 0;
            if idx_y + 1 < garden.len() {
                if region[idx_y + 1][idx_x] != region_number {
                    fences += 1;
                }
            }else {
                fences += 1;
            }

            if idx_y as i32 - 1 >= 0 {
                if region[idx_y - 1][idx_x] != region_number {
                    fences += 1;
                }
            }else {
                fences += 1;
            }

            if idx_x + 1 < garden[idx_y].len() {
                if region[idx_y][idx_x + 1] != region_number {
                    fences += 1;
                }
            }else {
                fences += 1;
            }

            if idx_x as i32 - 1 >= 0 {
                if region[idx_y][idx_x - 1] != region_number {
                    fences += 1;
                }
            }else {
                fences += 1;
            }
            area[region_number as usize] += 1;
            perimeter[region_number as usize] += fences;
        }
    } 

    for idx in 0..area.len() {
        cnt += area[idx] * perimeter[idx];
    }
    
    println!("C: {}", cnt);
}

fn print_matrix<R:std::fmt::Display>(matrix: Vec<Vec<R>>) {
    println!("\n...:");
    for line in matrix {
        for element in line {
            print!("{}", element);
        }
        println!("");
    }
}

fn breadth_search(idx_y: usize, idx_x: usize, region_cnt: u32, mut garden: Vec<Vec<char>>, mut region: Vec<Vec<u32>>) -> (Vec<Vec<char>>, Vec<Vec<u32>>) {
    let center: char = garden[idx_y][idx_x];
    region[idx_y][idx_x] = region_cnt;

    if idx_y + 1 < garden.len() && region[idx_y + 1][idx_x] == 0 && garden[idx_y + 1][idx_x] == center {
        (garden, region) = breadth_search(idx_y + 1, idx_x, region_cnt, garden, region);
    }
    if idx_y as i32 - 1 >= 0 && region[idx_y - 1][idx_x] == 0 && garden[idx_y - 1][idx_x] == center {
        (garden, region) = breadth_search(idx_y - 1, idx_x, region_cnt, garden, region);
    }
    if idx_x + 1 < garden[idx_y].len() && region[idx_y][idx_x + 1] == 0 && garden[idx_y][idx_x + 1] == center {
        (garden, region) = breadth_search(idx_y, idx_x + 1, region_cnt, garden, region);
    }
    if idx_x as i32 - 1 >= 0 && region[idx_y][idx_x - 1] == 0 && garden[idx_y][idx_x - 1] == center {
        (garden, region) = breadth_search(idx_y, idx_x - 1, region_cnt, garden, region);
    }

    return (garden, region);
}