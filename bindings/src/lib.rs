use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, rust!");
}

#[wasm_bindgen(start)]
pub fn on_start() {
    console_error_panic_hook::set_once();
}
