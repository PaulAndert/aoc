use std::fs;

#[derive(Debug, Clone)]
struct Bag {
    red: u64,
    green: u64,
    blue: u64,
}

pub fn main() {
    let contents = fs::read_to_string("./resources/day_02").expect("Should have been able to read the file");

    let mut cnt: u64 = 0;
    for line in contents.split("\n") {
        cnt += check_possible(line);
    }
    println!("C: {}", cnt);
}

fn check_possible(line: &str) -> u64 {
    let name_games: Vec<&str> = line.split(": ").collect();
    let games: Vec<&str> = name_games[1].split("; ").collect();

    let mut bag: Bag = Bag { red: 0, green: 0, blue: 0 };

    for game in games {
        let blocks: Vec<&str> = game.split(", ").collect();
        for block in blocks {
            let block_parts: Vec<&str> = block.split(" ").collect();
            let color: &str = &block_parts[1].replace(",", "");
            let count: u64 = block_parts[0].parse::<u64>().unwrap();

            match color {
                "red" => {
                    if count > bag.red {
                        bag.red = count;
                    }
                },
                "green" => {
                    if count > bag.green {
                        bag.green = count;
                    }
                },
                "blue" => {
                    if count > bag.blue {
                        bag.blue = count;
                    }
                },
                _ => {
                    println!("Error: unknown color |{}|", block_parts[1]);
                }
            }
        }
    }
    return bag.red * bag.green * bag.blue;
}