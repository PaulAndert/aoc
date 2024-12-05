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

    let mut incorrect_updates: Vec<Vec<String>> = Vec::new(); 
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
        if rule_broken {
            incorrect_updates.push(numbers);
        }
    }

    // fixin
    for mut update in incorrect_updates {

        let mut rule_breaks: u32 = 1;
        while rule_breaks > 0 {
            rule_breaks = 0;
            for rule in &rules {
                if update.contains(&rule.0) && update.contains(&rule.1) {
                    let idx_0 = update.iter().position(|a| a == &rule.0).unwrap();
                    let idx_1 = update.iter().position(|a| a == &rule.1).unwrap();
                    if idx_0 > idx_1 {
                        let temp = update[idx_0].clone();
                        update[idx_0] = update[idx_1].clone();
                        update[idx_1] = temp;
                        rule_breaks += 1;
                    }
                }
            }
        }
        cnt += update[update.len() / 2].parse::<u32>().unwrap();
    }
    
    println!("C: {}", cnt);
}