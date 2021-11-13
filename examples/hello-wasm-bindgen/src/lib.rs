use wasm_bindgen::prelude::*;

// wasm-pack requires "exported" functions to include #[wasm_bindgen]
#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn hello(src: &str) {
    alert(&format!("Hello, {}!", src));
}
