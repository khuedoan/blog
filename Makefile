.POSIX:

default: build

run:
	hugo server --buildDrafts

build:
	hugo --minify

new:
	hugo new --kind default posts/$(name).md
	nvim content/posts/$(name).md
