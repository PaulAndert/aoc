use std::fs;

pub fn main() {
    let contents = fs::read_to_string("./src/y2024/resources/day_05").expect("Should have been able to read the file");
    let mut cnt: u32 = 0;


    let mut rules: Vec<(String, String)> = Vec::new();
    let mut updates: Vec<String> = Vec::new();
    let mut switch: bool = true;
    for line in contents.split("\n") {
        if line == "" {
            switch = false;
            continue;
        }
        if switch {
            let mut pair = line.split("|");
            
            rules.push((pair.next().unwrap().to_string(), pair.next().unwrap().to_string()));
        }else {
            updates.push(line.to_string())
        }
    }

    for update in updates {
        let mut numbers: Vec<String> = Vec::new();
        for num in update.split(",") {
            numbers.push(num.to_string())
        }

        let mut rule_broken = false;
        for rule in &rules {
            if (numbers.contains(&rule.0) && numbers.contains(&rule.1)) &&
               (numbers.iter().position(|a| a == &rule.0).unwrap() > numbers.iter().position(|a| a == &rule.1).unwrap()) {
                rule_broken = true;
            }
        }
        if rule_broken == false {
            cnt += numbers[numbers.len() / 2].parse::<u32>().unwrap();
        }

    }
    
    println!("C: {}", cnt);
}