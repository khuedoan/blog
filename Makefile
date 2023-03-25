.POSIX:

default: build

node_modules/: package.json
	npm install

.PHONY:
dev: node_modules/
	npm run dev

.PHONY:
build: node_modules/
	npm run build

.PHONY:
lint: node_modules/
	npx prettier --check .
	npm run lint
