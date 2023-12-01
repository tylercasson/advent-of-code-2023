# A Rusty Advent of Code 2023

## Overview

This repository contains my solutions for the [Advent of Code 2023](https://adventofcode.com/2023) challenges, implemented in [Rust](https://www.rust-lang.org/).

## Structure

Each day's challenge is located in its own directory. The structure is as follows:

- `day-01`
- `day-02`
- ...
- `day-25`

## Generating a New Day

A new day can be generated from the template in the `cargo-template-day` directory using [`cargo-generate`](https://github.com/cargo-generate/cargo-generate):

```bash
cargo install cargo-generate # if not already installed
cargo generate --path cargo-template-day
```

## Running the Solutions

To run a solution, navigate to the respective day's directory, build the project, and run the desired part:

```bash
cd day-01
cargo build --release
./target/release/part1
./target/release/part2
```

## Benchmarking

I'm just using `perf` and `time` for basic CPU and memory measurement for now.

> Note that on some shells, you need to use the absolute path to `time` to avoid using the shell's built-in `time` command.

```bash
perf stat -r 10 ./target/release/part1 1>/dev/null
/usr/bin/time ./target/release/part1 1>/dev/null

perf stat -r 10 ./target/release/part2 1>/dev/null
/usr/bin/time ./target/release/part2 1>/dev/null
```

## License

This project is open-sourced under the [MIT License](LICENSE).
