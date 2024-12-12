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

#[derive(Clone, Debug)]
struct Direct {
    top: u32,
    right: u32,
    bottom: u32,
    left: u32
}

impl Direct {
    fn new() -> Self {
        Direct { top: 0, right: 0, bottom: 0, left: 0 }
    }
}

pub fn main_b() {
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

    let mut direction_matrix: Vec<Vec<Direct>> = Vec::new();
    for line in 0..garden.len() {
        direction_matrix.push(vec![Direct::new(); garden[line].len()]);
    }

    let mut sides_idx: u32 = 1;
    let mut area: Vec<u32> = vec![0; region_cnt as usize];
    for idx_y in 0..garden.len() {
        for idx_x in 0..garden[idx_y].len() {
            let region_number = region[idx_y][idx_x];

            // bottom
            if (idx_y + 1 < garden.len() && region[idx_y + 1][idx_x] != region_number) || (idx_y + 1 == garden.len()) {
                // right
                if idx_x + 1 < garden[idx_y].len() && region[idx_y][idx_x + 1] == region_number && direction_matrix[idx_y][idx_x + 1].bottom != 0 {
                    direction_matrix[idx_y][idx_x].bottom = direction_matrix[idx_y][idx_x + 1].bottom;
                    
                // left
                }else if idx_x as i32 - 1 >= 0 && region[idx_y][idx_x - 1] == region_number && direction_matrix[idx_y][idx_x - 1].bottom != 0 {
                    direction_matrix[idx_y][idx_x].bottom = direction_matrix[idx_y][idx_x - 1].bottom;
                
                // new side
                }else {
                    direction_matrix[idx_y][idx_x].bottom = sides_idx;
                    sides_idx += 1;
                }
            }

            // top
            if (idx_y as i32 - 1 >= 0 && region[idx_y - 1][idx_x] != region_number) || (idx_y as i32 - 1 == -1) {
                // right
                if idx_x + 1 < garden[idx_y].len() && region[idx_y][idx_x + 1] == region_number && direction_matrix[idx_y][idx_x + 1].top != 0 {
                    direction_matrix[idx_y][idx_x].top = direction_matrix[idx_y][idx_x + 1].top;
                    
                // left
                }else if idx_x as i32 - 1 >= 0 && region[idx_y][idx_x - 1] == region_number && direction_matrix[idx_y][idx_x - 1].top != 0 {
                    direction_matrix[idx_y][idx_x].top = direction_matrix[idx_y][idx_x - 1].top;
                
                // new side
                }else {
                    direction_matrix[idx_y][idx_x].top = sides_idx;
                    sides_idx += 1;
                }
            }

            // right
            if (idx_x + 1 < garden[idx_y].len() && region[idx_y][idx_x + 1] != region_number) || (idx_x + 1 == garden[idx_y].len()) {
                // bottom
                if idx_y + 1 < garden.len() && region[idx_y + 1][idx_x] == region_number && direction_matrix[idx_y + 1][idx_x].right != 0 {
                    direction_matrix[idx_y][idx_x].right = direction_matrix[idx_y + 1][idx_x].right;
                    
                // top
                }else if idx_y as i32 - 1 >= 0 && region[idx_y - 1][idx_x] == region_number && direction_matrix[idx_y - 1][idx_x].right != 0 {
                    direction_matrix[idx_y][idx_x].right = direction_matrix[idx_y - 1][idx_x].right;
                
                // new side
                }else {
                    direction_matrix[idx_y][idx_x].right = sides_idx;
                    sides_idx += 1;
                }
            }

            // left
            if (idx_x as i32 - 1 >= 0 && region[idx_y][idx_x - 1] != region_number) || (idx_x as i32 - 1 == -1) {
                // bottom
                if idx_y + 1 < garden.len() && region[idx_y + 1][idx_x] == region_number && direction_matrix[idx_y + 1][idx_x].left != 0 {
                    direction_matrix[idx_y][idx_x].left = direction_matrix[idx_y + 1][idx_x].left;
                    
                // top
                }else if idx_y as i32 - 1 >= 0 && region[idx_y - 1][idx_x] == region_number && direction_matrix[idx_y - 1][idx_x].left != 0 {
                    direction_matrix[idx_y][idx_x].left = direction_matrix[idx_y - 1][idx_x].left;
                
                // new side
                }else {
                    direction_matrix[idx_y][idx_x].left = sides_idx;
                    sides_idx += 1;
                }
            }

            area[region_number as usize] += 1;
        }
    } 

    let mut unique_sides: Vec<Vec<u32>> = Vec::new();
    for _line in 0..region_cnt {
        unique_sides.push(Vec::new());
    }

    for idx_y in 0..direction_matrix.len() {
        for idx_x in 0..direction_matrix[idx_y].len() {
            let current = direction_matrix[idx_y][idx_x].clone();
            let current_region = region[idx_y][idx_x];
            if current.top != 0 && !unique_sides[current_region as usize].contains(&current.top) {
                unique_sides[current_region as usize].push(current.top);
            }
            if current.right != 0 && !unique_sides[current_region as usize].contains(&current.right) {
                unique_sides[current_region as usize].push(current.right);
            }
            if current.bottom != 0 && !unique_sides[current_region as usize].contains(&current.bottom) {
                unique_sides[current_region as usize].push(current.bottom);
            }
            if current.left != 0 && !unique_sides[current_region as usize].contains(&current.left) {
                unique_sides[current_region as usize].push(current.left);
            }
        }
    } 

    for idx in 0..unique_sides.len() {
        cnt += unique_sides[idx].len() as u32 * area[idx];
    }
    
    println!("C: {}", cnt);
}

fn print_matrix<R:std::fmt::Display>(matrix: Vec<Vec<R>>) {
    println!("\n...:");
    for line in matrix {
        for element in line {
            print!("{} ", element);
        }
        println!("");
    }
}

fn print_direct_matrix(matrix: Vec<Vec<Direct>>) {
    println!("\n...:");
    for line in matrix {
        for element in line {
            print!("{:?} ", element.left);
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