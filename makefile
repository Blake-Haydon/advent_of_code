test: test-2023
build: build-2023

test-2023:
	cd aoc_2023 && cargo test --verbose

build-2023:
	cd aoc_2023 && cargo build --verbose
