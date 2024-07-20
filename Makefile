.POSIX:
.PHONY: default build dev lint fmt

default: build

dev:
	cargo run .

build:
	cargo build --release

lint:
	cargo clippy

fmt:
	cargo fmt
