use std::fmt::Display;
use std::fs::read_to_string;
use std::path::PathBuf;
use std::time::{Duration, Instant};

use comfy_table::{presets::UTF8_FULL_CONDENSED, Cell, CellAlignment, Table};

mod optimized;
#[allow(dead_code)]
mod solutions;
pub mod util;

fn main() {
    let dir = std::fs::read_dir("data").expect("no data directory");
    let mut profiles: Vec<_> = dir.map(|x| x.unwrap()).collect();
    profiles.sort_by_key(|profile| profile.path());
    for entry in profiles {
        let mut path = entry.path();
        println!("Profile: {}", path.file_name().unwrap().to_str().unwrap());
        path.push("inputs");
        let mut state = State::new(path);
        state.run_day(1, &optimized::day01::run);
        state.run_day(2, &solutions::day02::run);
        state.run_day(3, &solutions::day03::run);
        state.run_day(4, &solutions::day04::run);
        state.run_day(5, &solutions::day05::run);
        state.run_day(6, &optimized::day06::run);
        state.run_day(7, &solutions::day07::run);
        state.run_day(8, &solutions::day08::run);
        state.run_day(9, &solutions::day09::run);
        state.run_day(10, &solutions::day10::run);
        state.run_day(11, &optimized::day11::run);
        state.run_day(12, &solutions::day12::run);
        state.run_day(13, &optimized::day13::run);
        state.run_day(14, &optimized::day14::run);
        state.run_day(15, &optimized::day15::run);
        state.run_day(16, &optimized::day16::run);
        state.run_day(17, &solutions::day17::run);
        state.run_day(18, &solutions::day18::run);
        state.run_day(19, &optimized::day19::run);
        state.run_day(20, &solutions::day20::run);
        state.run_day(21, &solutions::day21::run);
        state.run_day(22, &solutions::day22::run);
        state.run_day(23, &optimized::day23::run);
        state.run_day(24, &optimized::day24::run);
        state.run_day(25, &solutions::day25::run);
        state.print();
    }
}

struct State {
    input_folder: PathBuf,
    table: Table,
    total: Duration,
    large_answers: Vec<String>,
}

impl State {
    fn new(input_folder: PathBuf) -> Self {
        let mut table = Table::new();
        table.load_preset(UTF8_FULL_CONDENSED);
        table.set_header(vec!["Day", "Part 1", "Part 2", "Time"]);
        Self { input_folder, table, total: Duration::ZERO, large_answers: Vec::new() }
    }

    fn run_day<A1: Display, A2: Display>(
        &mut self,
        day: usize,
        task: &dyn Fn(&str) -> (A1, A2),
    ) -> (A1, A2) {
        let mut path = self.input_folder.clone();
        path.push(format!("day{day:02}.txt"));
        // Read input
        let input = read_to_string(path)
            .unwrap_or_else(|_| panic!("missing input: {}", self.input_folder.display()));

        // Run solution
        let start = Instant::now();
        let (res1, res2) = task(input.as_str());
        let end = Instant::now();
        let delta = end.duration_since(start);

        // Check for long answers
        let mut res1_str = res1.to_string();
        if res1_str.lines().count() > 1 {
            self.large_answers
                .push(format!("Day {day:.2}, part 1:\n{res1_str}"));
            res1_str = String::from("see below");
        }

        let mut res2_str = res2.to_string();
        if res2_str.lines().count() > 1 {
            self.large_answers
                .push(format!("Day {day:.2}, part 2:\n{res2_str}"));
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
