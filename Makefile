.POSIX:

.PHONY: default
default: build

.PHONY: dev
dev:
	cargo leptos watch

.PHONY: build
build:
	cargo leptos build --release

.PHONY: fmt
fmt:
	cargo fmt
	leptosfmt .
