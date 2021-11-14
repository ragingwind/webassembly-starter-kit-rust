```sh
rustup target add wasm32-wasi
cargo build --target wasm32-wasi

# run with runtime
wasmtime --mapdir /host::. --invoke print_hello target/wasm32-wasi/debug/node_wasm.wasm

# run with node
node --experimental-wasi-unstable-preview1 --no-warnings index.mjs
```