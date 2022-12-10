use std::fmt::Display;
use std::time::{Duration, Instant};

use ascii::AsciiStr;
use comfy_table::{presets::UTF8_FULL_CONDENSED, Cell, CellAlignment, Table};

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;

fn main() {
    let mut state = State::new();
    state.run_day(&day1::run, "input/day1.txt");
    state.run_day(&day2::run, "input/day2.txt");
    state.run_day(&day3::run, "input/day3.txt");
    state.run_day(&day4::run, "input/day4.txt");
    state.run_day(&day5::run, "input/day5.txt");
    state.run_day(&day6::run, "input/day6.txt");
    state.run_day(&day7::run, "input/day7.txt");
    state.run_day(&day8::run, "input/day8.txt");
    state.run_day(&day9::run, "input/day9.txt");
    let (_, day10part2) = state.run_day(&day10::run, "input/day10.txt");

    state.print();
    println!("{day10part2:?}");
}

struct State {
    day_counter: usize,
    table: Table,
    total: Duration,
}

impl State {
    fn new() -> Self {
        let mut table = Table::new();
        table.load_preset(UTF8_FULL_CONDENSED);
        table.set_header(vec!["Day", "Part 1", "Part 2", "Time"]);
        Self {
            day_counter: 1,
            table,
            total: Duration::ZERO,
        }
    }

    fn run_day<A1: Display, A2: Display>(
        &mut self,
        task: &dyn Fn(&AsciiStr) -> (A1, A2),
        input_path: &str,
    ) -> (A1, A2) {
        let input = std::fs::read_to_string(input_path).unwrap();
        let input = AsciiStr::from_ascii(input.as_bytes()).unwrap();

        let start = Instant::now();
        let (res1, res2) = task(input);
        let end = Instant::now();
        let delta = end.duration_since(start);

        self.table.add_row(vec![
            Cell::new(self.day_counter.to_string()).set_alignment(CellAlignment::Right),
            Cell::new(res1.to_string()).set_alignment(CellAlignment::Right),
            Cell::new(res2.to_string()).set_alignment(CellAlignment::Right),
            Cell::new(format!("{delta:.2?}")).set_alignment(CellAlignment::Right),
        ]);
        self.total += delta;
        self.day_counter += 1;

        (res1, res2)
    }

    fn print(mut self) {
        self.table.add_row(vec!["", "", "", ""]);
        self.table.add_row(vec![
            Cell::new("Total"),
            Cell::new(""),
            Cell::new(""),
            Cell::new(format!("{:.2?}", self.total)).set_alignment(CellAlignment::Right),
        ]);
        println!("{}", self.table);
    }
}
