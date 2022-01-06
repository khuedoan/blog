.POSIX:

default: build

run:
	hugo server --buildDrafts

build:
	hugo --minify

new:
	hugo new --kind default posts/$(name).md
	${EDITOR} content/posts/$(name).md
