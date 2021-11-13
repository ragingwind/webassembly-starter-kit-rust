```sh
rustc --target wasm32-unknown-unknown -O ./hello-wasm.rs -o ./hello-wasm.wasm
serve .
```