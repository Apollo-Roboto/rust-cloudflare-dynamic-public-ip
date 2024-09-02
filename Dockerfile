FROM rust:1.80.1-bookworm AS build

WORKDIR /usr/src/rust-auto-public-ip-update
COPY . .
RUN cargo install --path .

FROM debian:bookworm-slim

RUN apt-get update
RUN apt-get install -y openssl ca-certificates
RUN rm -rf /var/lib/apt/lists/*

COPY --from=build /usr/local/cargo/bin/rust-auto-public-ip-update /usr/local/bin/apiu
ENTRYPOINT ["apiu"]
