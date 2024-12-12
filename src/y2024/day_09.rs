use std::fs;

pub fn main_a() {
    let contents = fs::read_to_string("./src/y2024/resources/day_09").expect("Should have been able to read the file");
    let mut cnt: u64 = 0;

    let mut memory: Vec<i32> = Vec::new();
    let mut idx = 0;
    let mut files = true;
    for char in contents.chars() {
        if !char.is_alphanumeric() {
            continue;
        }
        if files {
            for _i in 0..char as u32 - 48 {
                memory.push(idx);
            }
            idx += 1;
        }else {
            for _i in 0..char as u32 - 48 {
                memory.push(-1);
            }
        }
        files = !files;
    }

    let mut a: usize = 0;
    let mut b: usize = memory.len() - 1;
    while a < b {
        if memory[a] == -1 {
            memory[a] = memory[b];
            memory[b] = -1;
            b -= 1;
            while memory[b] == -1 {
                b -= 1;
            }
        }

        a += 1;
    }

    for i in 0..memory.len() {
        if memory[i] != -1 { cnt += i as u64 * memory[i] as u64; }
    }
    println!("C: {}", cnt);
}

