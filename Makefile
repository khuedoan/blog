.POSIX:

default: build

dev:
	trunk serve

build: src/styles/output.css
	cargo build --release

# TODO build Tailwind with cargo-leptos
# Need to install dependencies manually for now:
# npm i -D tailwindcss @tailwindcss/typography
src/styles/output.css: src/styles/globals.css
	npx tailwindcss -p @tailwindcss/typography -i src/styles/globals.css -o src/styles/output.css

.PHONY: format
format:
	cargo fmt
	leptosfmt .
