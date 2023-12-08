use std::collections::HashMap;
use std::fs;

pub fn main() {
    let contents = fs::read_to_string("./resources/day_08").expect("Should have been able to read the file");
    let mut lines: Vec<&str> = contents.split("\n").collect();

    let instructions: Vec<char> = lines[0].chars().collect();
    lines.remove(0);
    lines.remove(0);

    // get all paths with a &str connected to a (left, right) tuple HashMap
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

    // get all labels that are starts and ends
    let mut starts: Vec<&str> = Vec::new();
    let mut ends: Vec<&str> = Vec::new();
    for (key, _value) in &paths {
        if key.ends_with('A') {
            starts.push(key);
        } else if key.ends_with('Z') {
            ends.push(key);
        }
    }

    // get the steps required to go through a circle (from a Start to a End), for all Starts
    let mut circle_sizes: Vec<u64> = Vec::new();
    for start in starts {
        circle_sizes.push(calculate_circle_size(start, ends.clone(), paths.clone(), instructions.clone()));
    }

    // get the prime decomposition of every circle size
    let mut circle_primes: HashMap<u64, u64> = HashMap::new();
    for circle in &circle_sizes {
        for (key, new_value) in prime_decomposition(*circle) {
            match circle_primes.get(&key) {
                Some(old_value) => {
                    if new_value > *old_value {
                        let hash_c = circle_primes.entry(key).or_insert(0);
                        *hash_c = new_value;
                    }
                },
                None => {
                    circle_primes.entry(key).or_insert(new_value);
                }
            }
        }
    }

    // calculate the Least common multiple for all 6 start values
    // the step count where all meet the first time
    let mut product: u128 = 1;
    for (key, value) in &circle_primes {
        for i in 0..*value {
            product *= *key as u128;
        }
    }
    println!("Steps: {}", product);
}

fn prime_decomposition(mut circle: u64) -> HashMap<u64, u64> {
    let mut circle_primes: HashMap<u64, u64> = HashMap::new();
    let mut current_prime: u64 = 2;
    loop {
        if circle % current_prime == 0 {
            let hash_c = circle_primes.entry(current_prime).or_insert(0);
            *hash_c += 1;
            circle /= current_prime;
            if is_prime(circle) {
                let hash_c = circle_primes.entry(circle).or_insert(0);
                *hash_c += 1;
                return circle_primes;
            }
        }else {
            current_prime = generate_next_prime(current_prime);
        }
    }
}

// https://stackoverflow.com/questions/4475996/given-prime-number-n-compute-the-next-prime
fn generate_next_prime(mut old: u64) -> u64 {
    old += 1;
    if old <= 2 {
        return 2;
    }
    loop {
        if is_prime(old) {
            return old;
        }else {
            old += 1;
        }
    }
}
fn is_prime(x: u64) -> bool {
    for i in 2..1_000_000 {
        let q = x as f64 / i as f64;
        if q < i as f64 {
            return true;
        }
        if x == q as u64 * i {
            return false;
        }
    }
    return true;
}

fn calculate_circle_size<'a>(mut next: &'a str, ends: Vec<&str>, paths: HashMap<&str, (&'a str, &'a str)>, instructions: Vec<char>) -> u64 {
    let mut steps: u64 = 0;
    loop {
        let current: (&str, &str) = match paths.get(next) {
            Some(value) => *value,
            None => panic!("Error: unknown Key: {}", next),
        };
        match instructions[steps as usize % instructions.len()] {
            'L' => next = current.0,
            'R' => next = current.1,
            _ => panic!("Error: unknown Instruction: {}", instructions[steps as usize]),
        }

        steps += 1;
        if ends.contains(&next) {
            return steps;
        }
    }
}

// get circle sizes -> kleistes gemeinsames vielfaches