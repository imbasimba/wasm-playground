use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    let s = 18446744073709551615u64;
    alert(&format!("Hello, {}!", s));
}