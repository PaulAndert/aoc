use std::fs;

pub fn main() {
    let contents = fs::read_to_string("./src/y2024/resources/day_01").expect("Should have been able to read the file");
    
    let mut vec_a: Vec<u32> = Vec::new();
    let mut vec_b: Vec<u32> = Vec::new();

    for line in contents.split("\n") {
        let lists: Vec<&str> = line.split("   ").collect::<Vec<&str>>();
        if lists.len() == 2 {
            let numbers: Vec<u32> = lists.iter().map(|a| a.parse::<u32>().unwrap()).collect();
            vec_a.push(numbers[0]);
            vec_b.push(numbers[1]);
        }
    }

    vec_a.sort();
    vec_b.sort();

    let mut count = 0;
    for i in 0..vec_a.len() {
        let current_number = vec_a[i];
        count += current_number * (vec_b.iter().filter(|&&v| v == current_number).count()) as u32;
    }
    
    println!("C: {}", count);
}