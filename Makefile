
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

benchmark:
	/usr/bin/time cargo run --release

benchmark_each:
	for dir in $(SUBDIRS); do \
		echo "\nRunning $$dir"; \
		cd $$dir; \
		perf stat -r 10 cargo run -q --release 1>/dev/null; \
		cd ..; \
	done