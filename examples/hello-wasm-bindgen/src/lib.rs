use wasm_bindgen::prelude::*;

// wasm-pack requires "exported" functions to include #[wasm_bindgen]
#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn hello_alert(src: &str) {
    alert(&format!("Hello, {}!", src));
}

#[wasm_bindgen]
pub fn hello_dom() {
    let window = web_sys::window().expect("no global 'window' exists");
    let document = window.document().expect("should have a document on window");

    document.body().unwrap().set_text_content(Some("Hello, wasm-bindgen!"));
}
