.POSIX:

default: build

run:
	hugo server --buildDrafts

build:
	hugo

new:
	hugo new --kind default posts/$(name).md
	nvim content/posts/$(name).md

edit:
	nvim $(shell ls content/posts/*.md | fzf)

clean:
	git clean -xdf
