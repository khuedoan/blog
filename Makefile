.POSIX:

default: build

dev:
	trunk serve

build: styles/output.css
	cargo build --release

styles/output.css: styles/globals.css
	npx tailwindcss -i ./styles/globals.css -o ./styles/output.css
