use std::fs;

pub fn main() {
    let contents = fs::read_to_string("./src/y2024/resources/day_02").expect("Should have been able to read the file");

    let mut matrix: Vec<Vec<i32>> = Vec::new();
    for line in contents.split("\n") {
        let lists: Vec<&str> = line.split(" ").collect::<Vec<&str>>();
        if lists.len() > 1 {
            let numbers: Vec<i32> = lists.iter().map(|a| a.parse::<i32>().unwrap()).collect();
            matrix.push(numbers);
        }
    }

    let mut count = 0;
    for report in matrix {
        if evaluate_report(report.clone()) {
            count += 1
        }else if try_dampener(report) {
            count += 1
        }
    }
    
    println!("C: {}", count);
}

fn evaluate_report(report: Vec<i32>) -> bool {
    let direction: bool = (report[0] - report[1]) > 0;
    for idx in 0..report.len()-1 {
        let diff: i32 = report[idx] - report[idx + 1];
        if ((diff) > 0) != direction || diff.abs() < 1 || diff.abs() > 3 {
            return false;
        }
    }
    return true;
}

fn try_dampener(report: Vec<i32>) -> bool {
    for idx in 0..report.len() {
        let mut new_report = report.clone();
        new_report.remove(idx);

        if evaluate_report(new_report) {
            return true;
        }
    }
    return false;
}