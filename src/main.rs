use std::time::Instant;

mod days;

fn main() {
    let now = Instant::now();

    // days::day_01_a::main(); // to execute a day
    days::day_12_a::main();
    // cargo rustc -- -Awarnings && ./target/debug/aoc
    // to remove warnings

    println!("Elapsed: {:.2?}", now.elapsed());
}