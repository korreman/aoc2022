use std::time::Instant;

use comfy_table::{presets::UTF8_FULL_CONDENSED, Cell, CellAlignment, Table};

mod day1;
mod day2;
mod day3;

fn main() {
    let mut table = Table::new();
    table.load_preset(UTF8_FULL_CONDENSED);
    table.set_header(vec!["Day", "Part 1", "Part 2", "Time"]);

    let mut run_day = |day: u32, task: &dyn Fn(&str) -> (u32, u32), input_path: &str| {
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
    };

    run_day(1, &day1::run, "input/day1.txt");
    run_day(2, &day2::run, "input/day2.txt");
    run_day(3, &day3::run, "input/day3.txt");

    println!("{table}");
}
