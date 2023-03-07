FROM rust:1.67-alpine as build
WORKDIR /build
COPY . .
ENV RUSTFLAGS='-C target-feature=+crt-static' 
RUN apk add --no-cache build-base nodejs npm ca-certificates
RUN echo $(rustup show | head -n 1 | awk '{print $NF}') > /platform
RUN cargo build --release --target $(cat /platform) --bin boulder-timer
RUN mv target/$(cat /platform)/release/boulder-timer boulder-timer

FROM scratch
COPY --from=build \
    /etc/ssl/certs/ca-certificates.crt \
    /etc/ssl/certs/ca-certificates.crt
COPY --from=build /build/boulder-timer /boulder-timer
EXPOSE 3000
CMD ["/boulder-timer"]