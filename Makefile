.POSIX:

.PHONY: default
default: build

.PHONY: dev
dev:
	trunk serve

.PHONY: build
build: src/styles/output.css
	trunk build --release

src/styles/output.css: src/styles/globals.css
	tailwindcss --minify --input src/styles/globals.css --output src/styles/output.css

.PHONY: format
format:
	cargo fmt
	leptosfmt .
