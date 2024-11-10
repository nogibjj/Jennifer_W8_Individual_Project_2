# Makefile for Rust

BINARY_NAME=rust_sql

check:
	cd Rust_SQL && cargo check

build:
	cd Rust_SQL && cargo build

format:
	cd Rust_SQL && cargo fmt

lint:
	cd Rust_SQL && cargo clippy

test:
	cd Rust_SQL && cargo test

clean:
	cd Rust_SQL && cargo clean

release:
	cd Rust_SQL && cargo build --release
	@echo "Binary created at: Rust_SQL/target/release/$(BINARY_NAME)"

all: check build format lint test