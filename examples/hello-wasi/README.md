```sh
rustc --target wasm32-wasi -O ./hello-wasi.rs -o ./hello-wasi.wasm
wasmtime hello-wasi.wasm
```