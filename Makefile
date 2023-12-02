
SUBDIRS := $(shell ls | grep '^day-')

default: build_release

benchmark: build_release
	cargo bench -q --all-features

benchmark_each: build_release
	for dir in $(SUBDIRS); do \
		echo "\nRunning $$dir"; \
		cd $$dir; \
		cargo bench -q --all-features; \
		cd ..; \
	done

build:
	cargo build

build_release:
	cargo build --release

clean:
	cargo clean

clean_all:
	for dir in $(SUBDIRS); do \
		echo "\nCleaning $$dir"; \
		cd $$dir; \
		cargo clean; \
		cd ..; \
	done

generate:
	cargo generate --path cargo-template-day

perf: build_release
	perf stat -r 10 ./target/release/advent-of-code-2023 1>/dev/null

perf_each: build_release
	for dir in $(SUBDIRS); do \
		echo "\nRunning $$dir"; \
		cd $$dir; \
		cargo build --release; \
		perf stat -r 10 ./target/release/$$dir 1>/dev/null; \
		/usr/bin/time ./target/release/$$dir 1>/dev/null; \
		perf stat -r 10 ./target/release/part1 1>/dev/null; \
		/usr/bin/time ./target/release/part1 1>/dev/null; \
		perf stat -r 10 ./target/release/part2 1>/dev/null; \
		/usr/bin/time ./target/release/part2 1>/dev/null; \
		cd ..; \
	done

run:
	cargo run

run_release:
	cargo run --release
