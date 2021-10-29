extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Person {
    pub field1: HashMap<u32, String>,
    pub field2: Vec<Vec<f32>>,
    pub field3: String,
}

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

// Export a 'hello' function
#[wasm_bindgen]
pub fn hello(name: &str) {
    alert(&format!("Hello, {}!", name));
}

#[wasm_bindgen]
pub fn send_person_to_js() -> JsValue {
    let mut field1 = HashMap::new();
    field1.insert(0, String::from("ex"));
    let person = Person {
        field1,
        field2: vec![vec![1., 2.], vec![3., 4.]],
        field3: "shashank".to_string()
    };

    JsValue::from_serde(&person).unwrap()
}

#[wasm_bindgen]
pub fn receive_person_from_js(val: &JsValue) {
    let _example: Person = val.into_serde().unwrap();

}
