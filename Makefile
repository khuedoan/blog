.POSIX:

default: build

run:
	hugo server --buildDrafts

build:
	hugo
