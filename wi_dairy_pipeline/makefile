format:
	cargo fmt --quiet

lint:
	cargo clippy --quiet

test:
	cargo test --quiet

run:
	cargo run

# run CI steps in order
ci: format lint test

all: format lint test run