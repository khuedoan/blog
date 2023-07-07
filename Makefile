.POSIX:

.PHONY: default
default: build

.PHONY: dev
dev:
	trunk serve

.PHONY: build
build:
	trunk build --release

.PHONY: format
format:
	cargo fmt
	leptosfmt .
