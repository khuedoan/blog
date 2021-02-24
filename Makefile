.POSIX:

default: build

init:
	git submodule update --init --recursive

run:
	hugo server --buildDrafts

build: init
	hugo

deploy: build
	mv public /tmp
	git checkout gh-pages
	rm -rf *
	mv /tmp/public/* .
	git add --all
	git commit -m "Updates"
	git push

new:
	hugo new --kind default posts/$(name).md
	nvim content/posts/$(name).md

edit:
	nvim $(shell ls content/posts/*.md | fzf)

clean:
	git clean -xdf
