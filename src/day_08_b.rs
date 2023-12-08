use std::collections::HashMap;
use std::fs;

pub fn main() {
    let contents = fs::read_to_string("./resources/day_08").expect("Should have been able to read the file");
    let mut lines: Vec<&str> = contents.split("\n").collect();

    let instructions: Vec<char> = lines[0].chars().collect();
    lines.remove(0);
    lines.remove(0);

    let mut paths = HashMap::new();

    for mut line in lines {
        // remove last char
        line = &(line[0..line.len() - 1]);
        let split_equals: Vec<&str> = line.split(" = (").collect();
        let key: &str = split_equals[0];
        let split_comma: Vec<&str> = split_equals[1].split(", ").collect();

        let tuple_value: (&str, &str) = (
            split_comma[0], split_comma[1]
        );
        paths.insert(key, tuple_value);
    }

    let mut zzz_not_reached: bool = true;
    let mut steps: usize = 0;
    let mut next: &str = "AAA";
    while zzz_not_reached {
        let current: (&str, &str) = match paths.get(next) {
            Some(value) => *value,
            None => panic!("Error: unknown Key: {}", next),
        };
        match instructions[steps % instructions.len()] {
            'L' => next = current.0,
            'R' => next = current.1,
            _ => panic!("Error: unknown Instruction: {}", instructions[steps]),
        }

        steps += 1;
        if next == "ZZZ" {
            zzz_not_reached = false;
        }
    }
    println!("Reached after {} Steps", steps);
}