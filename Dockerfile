FROM rust:1.56.1-buster

RUN set -ex;
RUN apt-get update
RUN apt-get install --no-install-recommends -y binaryen wabt
RUN rustup target add wasm32-unknown-unknown
RUN mkdir -p /src

WORKDIR /src