extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

// Export a 'greetMe' function
#[wasm_bindgen]
pub fn greetMe(name: &str) {
    alert(&format!("Greetings, {}!", name));
}
