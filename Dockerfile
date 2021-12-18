FROM alpine as build

WORKDIR /usr/local/src/

RUN apk add \
    git \
    go \
    hugo

COPY . .

RUN hugo --minify

FROM nginx:alpine

COPY --from=build /usr/local/src/public /usr/share/nginx/html

EXPOSE 80
