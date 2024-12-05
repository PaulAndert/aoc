use std::time::Instant;

mod y2023;
mod y2024;

fn main() {
    let now = Instant::now();

    // days::day_01_a::main(); // to execute a day
    
    y2024::day_05_a::main();

    // cargo rustc -- -Awarnings && ./target/debug/aoc
    // to remove warnings

    println!("Elapsed: {:.2?}", now.elapsed());
}