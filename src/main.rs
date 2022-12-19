use std::fmt::Display;
use std::time::{Duration, Instant};

use comfy_table::{presets::UTF8_FULL_CONDENSED, Cell, CellAlignment, Table};

pub mod util;
mod solutions;
mod optimized;

fn main() {
    let mut state = State::new();
    state.run_day(01, &solutions::day01::run, "input/day01.txt");
    state.run_day(02, &solutions::day02::run, "input/day02.txt");
    state.run_day(03, &solutions::day03::run, "input/day03.txt");
    state.run_day(04, &solutions::day04::run, "input/day04.txt");
    state.run_day(05, &solutions::day05::run, "input/day05.txt");
    state.run_day(06, &solutions::day06::run, "input/day06.txt");
    state.run_day(07, &solutions::day07::run, "input/day07.txt");
    state.run_day(08, &solutions::day08::run, "input/day08.txt");
    state.run_day(00, &solutions::day09::run, "input/day09.txt");
    let (_, day10part2) = state.run_day(10, &solutions::day10::run, "input/day10.txt");
    state.run_day(11, &solutions::day11::run, "input/day11.txt");
    state.run_day(12, &solutions::day12::run, "input/day12.txt");
    state.run_day(13, &solutions::day13::run, "input/day13.txt");
    state.run_day(14, &solutions::day14::run, "input/day14.txt");
    state.run_day(15, &solutions::day15::run, "input/day15.txt");
    state.run_day(16, &solutions::day16::run, "input/day16.txt");
    state.run_day(17, &solutions::day17::run, "input/day17.txt");
    state.run_day(18, &solutions::day18::run, "input/day18.txt");
    state.run_day(19, &solutions::day19::run, "input/day19.txt");

    state.print();
    println!("{day10part2:?}");

    println!("Optimized solutions:");
    let mut state = State::new();
    state.run_day(01, &optimized::day01::run, "input/day01.txt");
    state.run_day(06, &optimized::day06::run, "input/day06.txt");
    state.run_day(11, &optimized::day11::run, "input/day11.txt");
    state.run_day(14, &optimized::day14::run, "input/day14.txt");
    state.run_day(15, &optimized::day15::run, "input/day15.txt");
    state.print();
}

struct State {
    table: Table,
    total: Duration,
}

impl State {
    fn new() -> Self {
        let mut table = Table::new();
        table.load_preset(UTF8_FULL_CONDENSED);
        table.set_header(vec!["Day", "Part 1", "Part 2", "Time"]);
        Self {
            table,
            total: Duration::ZERO,
        }
    }

    fn run_day<A1: Display, A2: Display>(
        &mut self,
        day: usize,
        task: &dyn Fn(&str) -> (A1, A2),
        input_path: &str,
    ) -> (A1, A2) {
        let input = std::fs::read_to_string(input_path).expect("missing input: {input_path}");

        let start = Instant::now();
        let (res1, res2) = task(input.as_str());
        let end = Instant::now();
        let delta = end.duration_since(start);

        self.table.add_row(vec![
            Cell::new(day.to_string()).set_alignment(CellAlignment::Right),
            Cell::new(res1.to_string()).set_alignment(CellAlignment::Right),
            Cell::new(res2.to_string()).set_alignment(CellAlignment::Right),
            Cell::new(format!("{delta:.2?}")).set_alignment(CellAlignment::Right),
        ]);
        self.total += delta;

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
