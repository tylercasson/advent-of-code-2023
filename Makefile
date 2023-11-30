
SUBDIRS := $(shell ls | grep '^day_')

default: build_release

build:
	cargo build

build_release:
	cargo build --release

run:
	cargo run

run_release:
	cargo run --release

clean:
	cargo clean

clean_all:
	for dir in $(SUBDIRS); do \
		echo "\nCleaning $$dir"; \
		cd $$dir; \
		cargo clean; \
		cd ..; \
	done

benchmark: build_release
	perf stat -r 10 ./target/release/advent_of_code_2023 1>/dev/null

benchmark_each: build_release
	for dir in $(SUBDIRS); do \
		echo "\nRunning $$dir"; \
		cd $$dir; \
		cargo build --release; \
		perf stat -r 10 ./target/release/$$dir 1>/dev/null; \
		/usr/bin/time ./target/release/$$dir 1>/dev/null; \
		cd ..; \
	done