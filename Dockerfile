FROM node:lts-alpine as build-client
WORKDIR /build
COPY client .
RUN npm install
RUN npm run build

FROM rust:1.67-alpine as build-server
WORKDIR /build
COPY . .
COPY --from=build-client /build/build client/build
RUN cargo build --release

FROM alpine:latest
COPY --from=build-server /build/target/release/boulder-timer /boulder-timer
CMD ["/boulder-timer"]