use aoc2022::*;
use ascii::AsciiStr;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

macro_rules! input {
    ($day:ident) => {
        let $day = std::fs::read_to_string(concat!("input/", stringify!($day), ".txt")).unwrap();
        let $day = AsciiStr::from_ascii($day.as_bytes()).unwrap();
    };
}

macro_rules! run {
    ($day:ident) => {
        $day::run(black_box($day))
    };
}

macro_rules! bench {
    ( $c:ident, $day:ident) => {
        $c.bench_function(stringify!($day), |b| b.iter(|| run!($day)));
    };
}

macro_rules! run_benchmarks {
    ( $c:ident, $($day:ident),+ ) => {
        $( input!($day); )+
        $( bench!($c, $day); )+
        $c.bench_function("all", |b| {b.iter(|| {
            $( run!($day); )+
        })});
    };
}

pub fn criterion_benchmark(c: &mut Criterion) {
    run_benchmarks!(
        c, day01, day02, day03, day04, day05, day06, day07, day08, day09, day10, day11, day12, day13
    );
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
