use std::fs;

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
    let game_id: u64 = name_games[0][5..name_games[0].len()].parse::<u64>().unwrap();

    let games: Vec<&str> = name_games[1].split("; ").collect();

    for game in games {
        let blocks: Vec<&str> = game.split(", ").collect();
        for block in blocks {
            let block_parts: Vec<&str> = block.split(" ").collect();
            let color: &str = &block_parts[1].replace(",", "");
            let count: u64 = block_parts[0].parse::<u64>().unwrap();

            match color {
                "red" => {
                    if count > 12 {
                        return 0;
                    }
                },
                "green" => {
                    if count > 13 {
                        return 0;
                    }
                },
                "blue" => {
                    if count > 14 {
                        return 0;
                    }
                },
                _ => {
                    println!("Error: unknown color |{}|", block_parts[1]);
                }
            }
        }
    }
    return game_id;
}