test: test-2023 test-2022
build: build-2023 build-2022

test-2022:
	cd aoc_2022 && cargo test --verbose
build-2022:
	cd aoc_2022 && cargo build --verbose

test-2023:
	cd aoc_2023 && cargo test --verbose
build-2023:
	cd aoc_2023 && cargo build --verbose


