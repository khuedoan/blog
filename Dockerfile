FROM node:19-slim as build
WORKDIR /usr/src/app
COPY . .
RUN npm run build

FROM nginx
COPY --from=build /usr/src/app/out /usr/share/nginx/html
