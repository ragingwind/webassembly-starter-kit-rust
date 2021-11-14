FROM ubuntu:20.04

ENV RUSTUP_HOME=/usr/local/rustup \
    CARGO_HOME=/usr/local/cargo \
    PATH=/usr/local/cargo/bin:$PATH \
    RUST_VERSION=1.56.1 \
    DEBIAN_FRONTEND=noninteractive \
    TZ=Asia/Seoul

RUN set -ex;

# Install development dependencies
RUN apt-get update && \
    apt-get install curl build-essential make gcc nodejs npm -y

# Install rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain=$RUST_VERSION
RUN chmod -R a+w $RUSTUP_HOME $CARGO_HOME
RUN rustup --version
RUN cargo --version
RUN rustc --version

# Install WASM targets
RUN rustup target add wasm32-unknown-unknown
RUN rustup target add wasm32-wasi

# Install WASM runtime, wasmtime
RUN curl https://wasmtime.dev/install.sh -sSf | bash

# Install wasm-pack
RUN curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh 

# HTTP serve for test
RUN npm install -g serve

# Tookits
RUN apt-get install --no-install-recommends -y binaryen wabt

WORKDIR /wask