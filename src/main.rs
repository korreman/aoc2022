use std::fmt::Display;
use std::time::Instant;

use comfy_table::{presets::UTF8_FULL_CONDENSED, Cell, CellAlignment, Table};

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;

fn main() {
    let mut table = Table::new();
    table.load_preset(UTF8_FULL_CONDENSED);
    table.set_header(vec!["Day", "Part 1", "Part 2", "Time"]);

    run_day(&mut table, 1, &day1::run, "input/day1.txt");
    run_day(&mut table, 2, &day2::run, "input/day2.txt");
    run_day(&mut table, 3, &day3::run, "input/day3.txt");
    run_day(&mut table, 4, &day4::run, "input/day4.txt");
    run_day(&mut table, 5, &day5::run, "input/day5.txt");

    println!("{table}");
}

fn run_day<A1: Display, A2: Display>(
    table: &mut Table,
    day: u32,
    task: &dyn Fn(&str) -> (A1, A2),
    input_path: &str,
) {
    let input = std::fs::read_to_string(input_path).unwrap();

    let start = Instant::now();
    let (res1, res2) = task(input.as_str());
    let end = Instant::now();
    let delta = end.duration_since(start);

    table.add_row(vec![
        Cell::new(day.to_string()).set_alignment(CellAlignment::Right),
        Cell::new(res1.to_string()).set_alignment(CellAlignment::Right),
        Cell::new(res2.to_string()).set_alignment(CellAlignment::Right),
        Cell::new(format!("{delta:.2?}")).set_alignment(CellAlignment::Right),
    ]);
}
