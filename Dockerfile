FROM google/cloud-sdk

EXPOSE 8080

ADD . /module-registry
WORKDIR module-registry

RUN cargo build

ENTRYPOINT cargo run
