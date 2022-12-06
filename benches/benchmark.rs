use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("Day 1", |b| {
        b.iter(|| aoc2022::day1::run(black_box(include_str!("../input/day1.txt"))))
    });
    c.bench_function("Day 2", |b| {
        b.iter(|| aoc2022::day2::run(black_box(include_str!("../input/day2.txt"))))
    });
    c.bench_function("Day 3", |b| {
        b.iter(|| aoc2022::day3::run(black_box(include_str!("../input/day3.txt"))))
    });
    c.bench_function("Day 4", |b| {
        b.iter(|| aoc2022::day4::run(black_box(include_str!("../input/day4.txt"))))
    });
    c.bench_function("Day 5", |b| {
        b.iter(|| aoc2022::day5::run(black_box(include_str!("../input/day5.txt"))))
    });
    c.bench_function("Day 6", |b| {
        b.iter(|| aoc2022::day6::run(black_box(include_str!("../input/day6.txt"))))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
