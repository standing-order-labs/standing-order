.PHONY: build test fmt lint check clean

build:
	./scripts/build.sh

test:
	cargo test --workspace

fmt:
	cargo fmt --all

lint:
	cargo clippy --workspace --all-targets -- -D warnings

check: fmt lint test

clean:
	cargo clean
