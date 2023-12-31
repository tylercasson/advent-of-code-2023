# A Rusty Advent of Code 2023

## Overview

This repository contains my solutions for the [Advent of Code 2023](https://adventofcode.com/2023) challenges, implemented in [Rust](https://www.rust-lang.org/). I'm still learning the language, so things might be a little messy for a while.

## Structure

Each day's challenge is located in its own directory. The structure is as follows:

- `day-01`
- `day-02`
- ...
- `day-25`

## Generating a New Day

A new day can be generated from the template in the `cargo-template-day` directory using [cargo-generate](https://github.com/cargo-generate/cargo-generate):

```bash
cargo install cargo-generate # if not already installed
cargo generate --path cargo-template-day
```

## Running the Solutions

To run a solution, navigate to the respective day's directory and run the desired part:

```bash
cd day-01
cargo run --bin day-01-part1
cargo run --bin day-01-part2
```

Or from the workspace root:

```bash
cargo run --bin day-01-part1
cargo run --bin day-01-part2
```

## Benchmarking

Days generated using the included template will be bootstrapped with [Criterion](https://github.com/bheisler/criterion.rs) and [Divan](https://github.com/nvzqz/divan) benchmarking.

> Refer to the ["**Generating a New Day**"](#generating-a-new-day) section for more info.

Benchmarks can be run in each day's directory:

```bash
cd day-01
cargo bench -q --all-features
```

Additionally, all days can be benchmarked from the workspace root:

```bash
cargo bench -q --all-features
```
