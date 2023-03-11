FROM rust:1.68-alpine as build
WORKDIR /build
COPY . .
ENV RUSTFLAGS='-C target-feature=+crt-static' 
ENV CARGO_REGISTRIES_CRATES_IO_PROTOCOL=sparse
RUN apk add --no-cache build-base nodejs npm ca-certificates
RUN echo $(rustup show | head -n 1 | awk '{print $NF}') > /platform
RUN cargo build --release --target $(cat /platform) --bin distributed-timer
RUN mv target/$(cat /platform)/release/distributed-timer distributed-timer

FROM scratch
COPY --from=build \
    /etc/ssl/certs/ca-certificates.crt \
    /etc/ssl/certs/ca-certificates.crt
COPY --from=build /build/distributed-timer /distributed-timer
EXPOSE 3000
CMD ["/distributed-timer"]