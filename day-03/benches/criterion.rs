use criterion::{black_box, criterion_group, criterion_main, Criterion};
use day_03::*;

pub fn part1(c: &mut Criterion) {
    let input = black_box(include_str!("../input/input1.txt"));
    c.bench_function("day_03::part1", |b| {
        b.iter(|| part1::run(input).expect("should benchmark part1"))
    });
}

pub fn part2(c: &mut Criterion) {
    let input = black_box(include_str!("../input/input1.txt"));
    c.bench_function("day_03::part2", |b| {
        b.iter(|| part2::run(input).expect("should benchmark part2"))
    });
}

criterion_group!(benches, part1, part2);
criterion_main!(benches);
