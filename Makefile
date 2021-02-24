.POSIX:

default: build

run:
	hugo server --buildDrafts

build:
	hugo

post:
	hugo new --kind default content/posts/$(name).md
