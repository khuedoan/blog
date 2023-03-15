.POSIX:

default: build

.PHONY:
dev: node_modules/
	npm run dev

.PHONY:
build: node_modules/
	npm run build

node_modules/: package.json package-lock.json
	npm install
