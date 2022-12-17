use aoc2022::*;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

macro_rules! input {
    ($day:ident) => {
        let $day = std::fs::read_to_string(concat!("input/", stringify!($day), ".txt")).unwrap();
    };
}

macro_rules! run {
    ( $module:ident, $day:ident) => {
        $module::$day::run(black_box($day.as_str()))
    };
}

macro_rules! bench {
    ( $c:ident, $module:ident, $day:ident) => {
        $c.bench_function(stringify!($day), |b| b.iter(|| run!($module, $day)));
    };
}

macro_rules! run_benchmarks {
    ( $c:ident, $module:ident, $($day:ident),+ ) => {
        $( input!($day); )+
        $( bench!($c, $module, $day); )+
        $c.bench_function("all", |b| {b.iter(|| {
            $( run!($module, $day); )+
        })});
    };
}

pub fn criterion_benchmark(c: &mut Criterion) {
    run_benchmarks!(
        c, solutions, day01, day02, day03, day04, day05, day06, day07, day08, day09, day10, day11,
        day12, day13, day14, day15, day17
    );
    run_benchmarks!(c, optimized, day01, day06, day14, day15);
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
