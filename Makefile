.POSIX:

default: build

run:
	hugo server --buildDrafts

build:
	hugo --minify

deploy: build
	git worktree add ./public gh-pages || true
	cd public \
		&& git commit --message "Updates" \
		&& git push

new:
	hugo new --kind default posts/$(name).md
	${EDITOR} content/posts/$(name).md
