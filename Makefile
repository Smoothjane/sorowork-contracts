.PHONY: build test fmt clippy clean

build:
	cargo build --target wasm32-unknown-unknown --release

test:
	cargo test --workspace

fmt:
	cargo fmt --all

fmt-check:
	cargo fmt --all -- --check

clippy:
	cargo clippy --all-targets -- -D warnings

clean:
	cargo clean

all: fmt-check clippy test build
