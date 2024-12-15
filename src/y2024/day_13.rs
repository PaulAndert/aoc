use std::{fs, result};

#[derive(Debug, Clone)]
struct Game {
    button_a_x: i64,
    button_a_y: i64,
    button_b_x: i64,
    button_b_y: i64,
    prize_x: i64,
    prize_y: i64
}


pub fn main_a() {
    let contents = fs::read_to_string("./src/y2024/resources/day_13").expect("Should have been able to read the file");
    let mut cnt: i64 = 0;

    let mut games: Vec<Game> = Vec::new();
    for block in contents.split("Button A") {
        let lines: Vec<&str> = block.split("\n").collect();
        if lines.len() < 3 {
            continue;
        }

        let a_line: Vec<&str> = lines[0].split(" ").collect();
        let a_1: &str = a_line[1];
        let a_2: &str = a_line[2];

        let b_line: Vec<&str> = lines[1].split(" ").collect();
        let b_1: &str = b_line[2];
        let b_2: &str = b_line[3];
        
        let prize_line: Vec<&str> = lines[2].split(" ").collect();
        let prize_1: &str = prize_line[1];
        let prize_2: &str = prize_line[2];

        games.push(Game{
            button_a_x: a_1[2..a_1.len()-1].parse::<i64>().unwrap(),
            button_a_y: a_2[2..].parse::<i64>().unwrap(),
            button_b_x: b_1[2..b_1.len()-1].parse::<i64>().unwrap(),
            button_b_y: b_2[2..].parse::<i64>().unwrap(),
            prize_x: prize_1[2..prize_1.len()-1].parse::<i64>().unwrap(),
            prize_y: prize_2[2..].parse::<i64>().unwrap()
        });
    }

    for game_idx in 0..games.len() {
        // if game_idx != 18 { continue; }
        let game = games[game_idx].clone();

        let x_steps_per_a_token = game.button_a_x / 3;
        let y_steps_per_a_token = game.button_a_y / 3;
        let x_steps_per_b_token = game.button_b_x / 1;
        let y_steps_per_b_token = game.button_b_y / 1;

        let a_is_cheaper = if x_steps_per_a_token + y_steps_per_a_token > x_steps_per_b_token + y_steps_per_b_token { true }else { false };

        println!("A = cheap -> {}", a_is_cheaper);

        let mut times_offset = 0;
        let mut best_solution: (i64, i64) = (0, 0);
        let mut best_cost: i64 = i64::max_value();
        while times_offset <= 100 {
            if a_is_cheaper {
                if game.prize_x % game.button_a_x == 0 {
                    if best_cost > (game.prize_x / game.button_a_x) * 3 {
                        best_solution = (game.prize_x / game.button_a_x, 0);
                        best_cost = game.prize_x / game.button_a_x * 3;
                    }
                }else {
                    let a_times_temp: i64 = game.prize_x / game.button_a_x;
                    if a_times_temp < times_offset {
                        break;
                    }
                    let a_times: i64 = if a_times_temp > 100 { 100 - times_offset } else { a_times_temp - times_offset };
                    let rest = game.prize_x - a_times * game.button_a_x;
                    if rest % game.button_b_x == 0 {
                        let b_count = rest / game.button_b_x;
                        if b_count <= 100 {
                            if best_cost > a_times * 3 + b_count {
                                if game.prize_y == a_times * game.button_a_y + b_count * game.button_b_y {
                                    best_solution = (a_times, b_count);
                                    best_cost = a_times * 3 + b_count;
                                }
                            }
                        }
                    }
                }
            }else {
                if game.prize_x % game.button_b_x == 0 {
                    if best_cost > 0 * 3 + game.prize_x / game.button_b_x {
                        best_solution = (0, game.prize_x / game.button_b_x);
                        best_cost = 0 * 3 + game.prize_x / game.button_b_x;
                    }
                }else {
                    let b_times_temp: i64 = game.prize_x / game.button_b_x;
                    if b_times_temp < times_offset {
                        break;
                    }
                    let b_times: i64 = if b_times_temp > 100 { 100 - times_offset } else { b_times_temp - times_offset };
                    let rest = game.prize_x - b_times * game.button_b_x;
                    println!("TO: {} -> BT: {} -> R: {}", times_offset, b_times, rest);
                    println!("1 {}", rest % game.button_a_x);
                    if rest % game.button_a_x == 0 {
                        let a_count= rest / game.button_a_x;
                        println!("2 {} <= 100 = {}", a_count, a_count <= 100);
                        if a_count <= 100 {
                            println!("3 {} > {}", best_cost,  a_count * 3 + b_times);
                            if best_cost > a_count * 3 + b_times {
                                println!("4 {} == {}", game.prize_y, a_count * game.button_a_y + b_times * game.button_b_y);
                                println!("AB {} {}", a_count, b_times);
                                if game.prize_y == a_count * game.button_a_y + b_times * game.button_b_y {
                                    println!("5 HIT");
                                    best_solution = (a_count, b_times);
                                    best_cost = a_count * 3 + b_times;
                                }
                            }
                        }
                    }
                }
            }
            times_offset += 1; 
        }

        if best_cost != i64::max_value() && best_solution.0 != 0 && best_solution.1 != 0 {
            cnt += best_cost;
        }else {
            let (a, b) = calculate(game.clone());
            if a != 0 && b != 0 {
                println!("ERROR: {}: {:?} {}|{}", game_idx, game, a, b);
            }
        }
    }

    println!("G: {}", games.len());
    println!("C: {}", cnt);
}

pub fn main_a_2() {
    let contents = fs::read_to_string("./src/y2024/resources/day_13").expect("Should have been able to read the file");
    let mut cnt: i64 = 0;

    let mut games: Vec<Game> = Vec::new();
    for block in contents.split("Button A") {
        let lines: Vec<&str> = block.split("\n").collect();
        if lines.len() < 3 {
            continue;
        }

        let a_line: Vec<&str> = lines[0].split(" ").collect();
        let a_1: &str = a_line[1];
        let a_2: &str = a_line[2];

        let b_line: Vec<&str> = lines[1].split(" ").collect();
        let b_1: &str = b_line[2];
        let b_2: &str = b_line[3];
        
        let prize_line: Vec<&str> = lines[2].split(" ").collect();
        let prize_1: &str = prize_line[1];
        let prize_2: &str = prize_line[2];

        games.push(Game{
            button_a_x: a_1[2..a_1.len()-1].parse::<i64>().unwrap(),
            button_a_y: a_2[2..].parse::<i64>().unwrap(),
            button_b_x: b_1[2..b_1.len()-1].parse::<i64>().unwrap(),
            button_b_y: b_2[2..].parse::<i64>().unwrap(),
            prize_x: prize_1[2..prize_1.len()-1].parse::<i64>().unwrap(),
            prize_y: prize_2[2..].parse::<i64>().unwrap()
        });
    }

    for game_idx in 0..games.len() {
        let game = games[game_idx].clone();

        let (a, b) = calculate(game);
        if a != 0 && b != 0 {
            cnt += (a * 3 + b) as i64;
        }
    }

    println!("G: {}", games.len());
    println!("C: {}", cnt);
}


fn calculate(game: Game) -> (i64, i64) {
    let shift: i64 = 10000000000000;
    let det: i64 = (game.button_a_x * game.button_b_y) - (game.button_a_y * game.button_b_x);
        let a: i64 = (((game.prize_x + shift) * game.button_b_y) - ((game.prize_y + shift) * game.button_b_x)) / det;
        let b: i64 = ((game.button_a_x * (game.prize_y + shift)) - (game.button_a_y * (game.prize_x + shift))) / det;

        if a * game.button_a_x + b * game.button_b_x == (game.prize_x + shift)
        && a * game.button_a_y + b * game.button_b_y == (game.prize_y + shift) {
            return (a, b);
        }else {
            return (0, 0);
        }
}

// 29761
// ?? -> 27105
// 25041
// 24783