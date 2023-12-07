use std::time::Instant;

mod day {
    include!("day_06_b.rs");
}

fn main() {
    let now = Instant::now();
    day::main();
    println!("Elapsed: {:.2?}", now.elapsed());
}