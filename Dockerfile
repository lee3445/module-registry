FROM rust:latest

ADD . /module-registry
WORKDIR module-registry

RUN git submodule update --init --recursive
RUN cargo build

CMD cargo run
