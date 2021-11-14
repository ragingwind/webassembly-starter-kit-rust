# with-simd

> SIMD for rust-lang and WebAssembly

# How to use

```sh
# toolchain update
rustup toolchain install nightly-2021-07-29 && rustup target add wasm32-wasi --toolchain=nightly-2021-07-29

# build
RUSTFLAGS='-C target-feature=+simd128' cargo +nightly-2021-07-29 build --release --target=wasm32-wasi

# run with simd
wasmtime run --wasm-features=all ./target/wasm32-wasi/release/with-simd.wasm 
```

# License

@ MIT
