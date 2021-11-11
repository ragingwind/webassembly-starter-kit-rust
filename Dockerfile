FROM ubuntu:20.04

ENV RUSTUP_HOME=/usr/local/rustup \
    CARGO_HOME=/usr/local/cargo \
    PATH=/usr/local/cargo/bin:$PATH \
    RUST_VERSION=1.56.1

RUN set -ex;

RUN apt-get update
RUN apt-get install curl build-essential make gcc -y

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain=$RUST_VERSION
RUN chmod -R a+w $RUSTUP_HOME $CARGO_HOME
RUN rustup --version
RUN cargo --version
RUN rustc --version

RUN apt-get install --no-install-recommends -y binaryen wabt
RUN rustup target add wasm32-unknown-unknown
RUN mkdir -p /src

WORKDIR /src