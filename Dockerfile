FROM rust:latest

EXPOSE 8080

ADD . /module-registry
WORKDIR module-registry

RUN cargo build

ENTRYPOINT cargo run
