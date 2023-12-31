use criterion::{black_box, criterion_group, criterion_main, Criterion};
use day_05::*;

pub fn part1(c: &mut Criterion) {
    let input = black_box(include_str!("../input/input1.txt"));
    c.bench_function("day_05::part1", |b| {
        b.iter(|| part1::run(input).expect("should benchmark part1"))
    });
}

pub fn part2(c: &mut Criterion) {
    let input = black_box(include_str!("../input/input1.txt"));
    let mut group = c.benchmark_group("very slow part2");

    group.sample_size(10);

    group.bench_function("day_05::part2", |b| {
        b.iter(|| part2::run(input).expect("should benchmark part2"))
    });

    group.finish();
}

pub fn part2_a(c: &mut Criterion) {
    let input = black_box(include_str!("../input/input1.txt"));
    c.bench_function("day_05::part2_a", |b| {
        b.iter(|| part2_a::run(input).expect("should benchmark part2_a"))
    });
}

criterion_group!(benches, part1, part2_a);
criterion_main!(benches);
