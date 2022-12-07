use aoc2022::*;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

macro_rules! input {
    ($day:ident) => {
        let $day =
            std::fs::read_to_string(concat!("input/", stringify!($day), ".txt")).unwrap();
    };
}

macro_rules! run {
    ($day:ident) => {
        $day::run(black_box($day.as_str()))
    };
}

macro_rules! bench {
    ( $c:ident, $day:ident) => {
        $c.bench_function(stringify!($day), |b| {
            b.iter(|| run!($day))
        });
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
    run_benchmarks!(c, day1, day2, day3, day4, day5, day6, day7);
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
