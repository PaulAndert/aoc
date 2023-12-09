use std::time::Instant;

mod day {
    include!("day_09_a.rs");
}

fn main() {
    let now = Instant::now();
    day::main();
    println!("Elapsed: {:.2?}", now.elapsed());
}