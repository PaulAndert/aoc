use std::time::Instant;

mod days;

fn main() {
    let now = Instant::now();

    // days::day_01_a::main(); // to execute a day
    days::day_10_a::main();

    println!("Elapsed: {:.2?}", now.elapsed());
}