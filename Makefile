.POSIX:
.PHONY: default dev fmt lint test test-unit test-load container update ci

default: fmt lint target/release

target/release: src/ Cargo.toml Cargo.lock
	cargo leptos build --release

dev:
	bacon run

fmt:
	cargo fmt
	leptosfmt src/**/*.rs

lint:
	cargo clippy -- --deny warnings
	cargo fmt --check
	leptosfmt src/**/*.rs --check

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

ci:
	@mkdir -p "${CACHE_DIR}/target"
	@ln -s "${CACHE_DIR}/target" "target"
	@make test
