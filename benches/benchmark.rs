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
    ( $c:ident, $(($module:ident, $day:ident)),+ ) => {
        $( input!($day); )+
        $( bench!($c, $module, $day); )+
        $c.bench_function("all", |b| {b.iter(|| {
            $( run!($module, $day); )+
        })});
    };
}

pub fn criterion_benchmark(c: &mut Criterion) {
    run_benchmarks!(
        c,
        (optimized, day01),
        (solutions, day02),
        (solutions, day03),
        (solutions, day04),
        (solutions, day05),
        (optimized, day06),
        (solutions, day07),
        (solutions, day08),
        (solutions, day09),
        (solutions, day10),
        (optimized, day11),
        (solutions, day12),
        (optimized, day13),
        (optimized, day14),
        (optimized, day15),
        (optimized, day16),
        (solutions, day17),
        (solutions, day18),
        (optimized, day19),
        (solutions, day20),
        (solutions, day21),
        (solutions, day22),
        (solutions, day23),
        (optimized, day24),
        (solutions, day25)
    );
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
