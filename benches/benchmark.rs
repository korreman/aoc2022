use aoc2022::*;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

macro_rules! bench {
    ( $c:ident, $day:ident) => {
        $c.bench_function(stringify!($day), |b| {
            let input =
                std::fs::read_to_string(concat!("input/", stringify!($day), ".txt")).unwrap();
            b.iter(|| $day::run(black_box(input.as_str())))
        });
    };
}

pub fn criterion_benchmark(c: &mut Criterion) {
    bench!(c, day1);
    bench!(c, day2);
    bench!(c, day3);
    bench!(c, day4);
    bench!(c, day5);
    bench!(c, day6);
    bench!(c, day7);
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
