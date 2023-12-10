use day_10::*;

fn main() {
    divan::main();
}

#[divan::bench]
fn part1() {
    let input = divan::black_box(include_str!("../input/input1.txt"));
    part1::run(input).expect("should benchmark part 1");
}

#[divan::bench]
fn part2() {
    let input = divan::black_box(include_str!("../input/input1.txt"));
    part2::run(input).expect("should benchmark part 2");
}
