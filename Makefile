.POSIX:
.PHONY: default dev fmt lint test test-unit test-load container update ci

default: fmt lint target/release/blog

target/release/blog: src/ Cargo.toml
	cargo build --release

dev:
	cargo watch --exec 'run'

fmt:
	cargo fmt

lint:
	cargo clippy -- --deny warnings

test: lint test-unit

test-unit:
	cargo test

test-load:
	k6 run tests/load/index.js

container:
	docker build . --tag blog

update:
	nix flake update
	cargo update
	curl --silent --show-error --location 'https://cdn.jsdelivr.net/npm/@picocss/pico@2/css/pico.indigo.min.css' --output public/pico.min.css
	curl --silent --show-error --location 'https://unpkg.com/htmx.org@2.0.1' --output public/htmx.min.js

ci:
	make test
