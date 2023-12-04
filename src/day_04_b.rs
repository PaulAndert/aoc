use std::fs;

pub fn main() {
    let contents = fs::read_to_string("./resources/day_04").expect("Should have been able to read the file");
    let lines: Vec<&str> = contents.split("\n").collect();

    let mut cards: Vec<u64> = Vec::new();
    for _i in 0..lines.len() {
        cards.push(1);
    }

    let mut cnt: usize = 0;
    for line in lines {
        let wins = get_card_value(line);
        for i in (cnt+1)..=(cnt+wins as usize) {
            cards[i] += cards[cnt];
        }
        cnt += 1;
    }
    let sum: u64 = cards.iter().sum();
    println!("S: {}", sum);
}

fn get_card_value(line: &str) -> u64 {
    let numbers: &str = line.split(": ").collect::<Vec<&str>>()[1];

    let parts: Vec<&str> = numbers.split(" | ").collect();
    let winning: Vec<u64> = str_vec_to_u64_vec(parts[0].split(" ").collect::<Vec<&str>>());
    let you_have: Vec<u64> = str_vec_to_u64_vec(parts[1].split(" ").collect::<Vec<&str>>());

    let mut cnt: u64 = 0;
    for val in you_have {
        if winning.contains(&val) {
            cnt += 1;
        }
    }
    cnt
}

fn str_vec_to_u64_vec(strings: Vec<&str>) -> Vec<u64> {
    let mut ret: Vec<u64> = Vec::new();

    for mut string in strings {
        let string_adjusted = &string.replace(" ", "");
        if string_adjusted == "" {
            continue;
        }
        let mut value: u64 = 0;
        let mut index: u32 = 0;
        for c in string_adjusted.chars().rev() {
            value += (c as u64 - '0' as u64) * u64::pow(10, index);
            index += 1;   
        }
        ret.push(value);
    }
    ret
}