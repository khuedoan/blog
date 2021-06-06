.POSIX:

TAG=khuedoan/khuedoan.com

default: build

build:
	docker build . -t ${TAG}

run: build
	docker run --publish 1313:80 --rm ${TAG}
