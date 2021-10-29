//extern crate wasm_bindgen;
use cuckoofilter::CuckooFilter;
use serde::{Deserialize, Serialize};
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
extern crate wapc_guest as guest;
pub use generated::*;
use guest::prelude::*;

lazy_static! {
    static ref cf: CuckooFilter<DefaultHasher> = {
        let CF: CuckooFilter<DefaultHasher> = load_data();
        CF
    };
}

fn load_data() -> CuckooFilter<DefaultHasher> {
    let words = vec!["foo", "bar", "xylophone", "milagro"];
    let mut CF2: CuckooFilter<DefaultHasher> = CuckooFilter::new();

    let mut insertions = 0;
    for s in &words {
        if CF2.test_and_add(s) {
            insertions += 1;
        }
    }
    CF2
}

fn check_word_exists(member: String) -> HandlerResult<String> {
    let exists = cf.contains(&member);
    println!("{}", exists);
    Ok(exists.to_string())
}


struct Input {
    x: String,
    y: String,
}

#[no_mangle]
pub fn wapc_init() {
    Handlers::register_check_word_exists(check_word_exists);
    Handlers::register_handle_input(handle_input);
}
fn handle_input(inp: generated::Input) -> HandlerResult<String> {
    let a = &inp.x;
    Ok(a.to_string())
}
