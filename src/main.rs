mod day_10_a;

use std::time::Instant;

mod day {
    include!("day_09_b.rs");
}

fn main() {
    let now = Instant::now();
    day::main();
    println!("Elapsed: {:.2?}", now.elapsed());
}