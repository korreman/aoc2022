use std::time::Instant;

mod day1;
mod day2;
mod day3;

fn main() {
    run(day1::run, "input/day1.txt", "Day 1");
    run(day2::run, "input/day2.txt", "Day 2");
    run(day3::run, "input/day3.txt", "Day 3");
}

fn run<R: std::fmt::Display, T: Fn(&str) -> (R, R)>(task: T, input_path: &str, name: &str) {
    let input = std::fs::read_to_string(input_path).unwrap();

    let start = Instant::now();
    let (res1, res2) = task(input.as_str());
    let end = Instant::now();

    let delta = end.duration_since(start);
    println!(
        "{} | Part 1: {} | Part 2: {} | Time: {:?}",
        name,
        res1,
        res2,
        delta
    );
}
