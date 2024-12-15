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


pub fn main_b() {
    let contents = fs::read_to_string("./src/y2024/resources/day_09").expect("Should have been able to read the file");
    let mut cnt: u64 = 0;

    // (count, number)
    let mut memory: Vec<(usize, i32)> = Vec::new();
    let mut idx = 0;
    let mut files = true;
    for char in contents.chars() {
        if !char.is_alphanumeric() {
            continue;
        }
        let num = char as usize - 48;
        if files {
            memory.push((num, idx));
            idx += 1;
        }else {
            if num != 0 {
                memory.push((num, -1));
            }
        }
        files = !files;
    }

    for i in 0..memory.len() {
        print!("{:?} ", memory[i]);
    }
    println!("");

    let mut a: usize = 0;
    let mut b: usize = memory.len() - 1;
    let mut idx = 0;
    while b > 0 {
        
    }
    


    for i in 0..memory.len() {
        print!("{:?} ", memory[i]);
    }
    println!("");

    // for i in 0..memory.len() {
    //     if memory[i] != -1 { cnt += i as u64 * memory[i] as u64; }
    // }
    println!("");
    println!("C: {}", cnt);
}
















// loop {
//     println!("\n{}: A({}): {:?} :: B({}): {:?}", idx, a, memory[a], b, memory[b]);


//     if memory[a].1 == -1 {
//         if memory[a].0 >= memory[b].0 {
//             println!("HIT");
//             let gap = memory[a].0 as i32 - memory[b].0 as i32;
//             memory[a] = memory[b];
//             println!("Move {:?}", memory[b]);
//             if gap > 0 {
//                 println!("Gap: {}", gap);
//                 memory.splice(a+1..a+1, vec![(gap as usize, -1)].iter().cloned());

//                 for i in 0..memory.len() {
//                     print!("{:?} ", memory[i]);
//                 }
//                 println!("");
//                 b += 1;
//             }
            
//             memory.remove(b);
//             a = 0;
//             while memory[a].1 == -1 {
//                 a += 1;
//             }
//             println!("reset A");
//             b = memory.len() - 1;
//             while memory[b].1 == -1 {
//                 b -= 1;
//             }
//             println!("reset B {}", b);
//         }else {
//             println!("upper A");
//             a += 1;
//             while memory[a].1 == -1 {
//                 if a+1 < memory.len() {
//                     a += 1;
//                 }else {
//                     a = 0;
//                     b -= 1;
//                     while memory[b].1 == -1 {
//                         b -= 1;
//                     }
//                     break;
//                 }
//             }
//             continue;
//         }

//     }else {
//         a += 1;
//     }


//     let mut new_memory: Vec<(usize, i32)> = Vec::new();
//     for idx in 0..memory.len() {
//         if memory[idx].1 == -1 && idx + 1 < memory.len() && memory[idx + 1].1 == -1 {
//             new_memory.push((memory[idx].0 + memory[idx + 1].0, -1));
//             a -= 1;
//             b -= 1;
//         }else {
//             new_memory.push(memory[idx]);
//         }
//     }
//     memory = new_memory;

//     for i in 0..memory.len() {
//         print!("{:?} ", memory[i]);
//     }
//     println!("");
//     idx += 1;
//     if b == 0 || idx == 150{
//        break; 
//     }
// }