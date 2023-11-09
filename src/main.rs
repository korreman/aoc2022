use std::fmt::Display;
use std::time::{Duration, Instant};

use comfy_table::{presets::UTF8_FULL_CONDENSED, Cell, CellAlignment, Table};

pub mod util;
#[allow(dead_code)]
mod solutions;
mod optimized;

fn main() {
    let mut state = State::new();
    state.run_day(01, &optimized::day01::run, "data/a/inputs/day01.txt");
    state.run_day(02, &solutions::day02::run, "data/a/inputs/day02.txt");
    state.run_day(03, &solutions::day03::run, "data/a/inputs/day03.txt");
    state.run_day(04, &solutions::day04::run, "data/a/inputs/day04.txt");
    state.run_day(05, &solutions::day05::run, "data/a/inputs/day05.txt");
    state.run_day(06, &optimized::day06::run, "data/a/inputs/day06.txt");
    state.run_day(07, &solutions::day07::run, "data/a/inputs/day07.txt");
    state.run_day(08, &solutions::day08::run, "data/a/inputs/day08.txt");
    state.run_day(09, &solutions::day09::run, "data/a/inputs/day09.txt");
    state.run_day(10, &solutions::day10::run, "data/a/inputs/day10.txt");
    state.run_day(11, &optimized::day11::run, "data/a/inputs/day11.txt");
    state.run_day(12, &solutions::day12::run, "data/a/inputs/day12.txt");
    state.run_day(13, &optimized::day13::run, "data/a/inputs/day13.txt");
    state.run_day(14, &optimized::day14::run, "data/a/inputs/day14.txt");
    state.run_day(15, &optimized::day15::run, "data/a/inputs/day15.txt");
    state.run_day(16, &optimized::day16::run, "data/a/inputs/day16.txt");
    state.run_day(17, &solutions::day17::run, "data/a/inputs/day17.txt");
    state.run_day(18, &solutions::day18::run, "data/a/inputs/day18.txt");
    state.run_day(19, &optimized::day19::run, "data/a/inputs/day19.txt");
    state.run_day(20, &solutions::day20::run, "data/a/inputs/day20.txt");
    state.run_day(21, &solutions::day21::run, "data/a/inputs/day21.txt");
    state.run_day(22, &solutions::day22::run, "data/a/inputs/day22.txt");
    state.run_day(23, &optimized::day23::run, "data/a/inputs/day23.txt");
    state.run_day(24, &optimized::day24::run, "data/a/inputs/day24.txt");
    state.run_day(25, &solutions::day25::run, "data/a/inputs/day25.txt");
    state.print();
}

struct State {
    table: Table,
    total: Duration,
    large_answers: Vec<String>,
}

impl State {
    fn new() -> Self {
        let mut table = Table::new();
        table.load_preset(UTF8_FULL_CONDENSED);
        table.set_header(vec!["Day", "Part 1", "Part 2", "Time"]);
        Self {
            table,
            total: Duration::ZERO,
            large_answers: Vec::new(),
        }
    }

    fn run_day<A1: Display, A2: Display>(
        &mut self,
        day: usize,
        task: &dyn Fn(&str) -> (A1, A2),
        input_path: &str,
    ) -> (A1, A2) {
        // Read input
        let input = std::fs::read_to_string(input_path).expect("missing input: {input_path}");

        // Run solution
        let start = Instant::now();
        let (res1, res2) = task(input.as_str());
        let end = Instant::now();
        let delta = end.duration_since(start);

        // Check for long answers
        let mut res1_str = res1.to_string();
        if res1_str.lines().count() > 1 {
            self.large_answers.push(format!("Day {day:.2}, part 1:\n{res1_str}"));
            res1_str = String::from("see below");
        }

        let mut res2_str = res2.to_string();
        if res2_str.lines().count() > 1 {
            self.large_answers.push(format!("Day {day:.2}, part 2:\n{res2_str}"));
            res2_str = String::from("see below");
        }

        // Add answers to table
        self.table.add_row(vec![
            Cell::new(day.to_string()).set_alignment(CellAlignment::Right),
            Cell::new(res1_str).set_alignment(CellAlignment::Right),
            Cell::new(res2_str).set_alignment(CellAlignment::Right),
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
        println!("{}\n", self.table);

        // print long solutions after table
        for answer in self.large_answers {
            println!("{answer}\n");
        }
    }
}
