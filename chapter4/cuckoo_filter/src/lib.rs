#[macro_use]
extern crate lazy_static;
extern crate wasm_bindgen;
extern crate cuckoofilter;
use wasm_bindgen::prelude::*;
use std::collections::HashMap;

// this library allows us to serialize and deserialize the json. Our js code will pass json as input to wasm //module which will deserialize it.
use serde::{Serialize, Deserialize};
// existing cuckoo filter crate
use cuckoofilter::CuckooFilter;
use std::collections::hash_map::DefaultHasher;

// lazy_static! Code snippet allows the Rust program to hold a global reference. Here we need to create 
// a cuckoo filter Data structure as a global variable, which we will then use across our code. This //structure is initialized at startup with pre existing keys
lazy_static! {

static ref cf:CuckooFilter<DefaultHasher> = {
let  CF:CuckooFilter<DefaultHasher>=load_data();
CF
};
}
// method for loading the keys. Here we have just loaded 4 but these entries can easily be in millions
//As the  goal here is to show a working of wasm module majorly and not Bloom Filter per say, we will
// keep the entries limited to 4. This method was invoked from within the lazy static method which then 
// exposes a global data structure CF for the cuckoo filter.
fn load_data() -> CuckooFilter<DefaultHasher>
{
let words = vec!["foo", "bar", "xylophone", "milagro"];
// mut keyword allows a mutable variable to be declared in Rust. Since we need our structure to be 
// mutable.
let mut CF2:CuckooFilter<DefaultHasher> = CuckooFilter::new();

let mut insertions = 0;
// Iterate over the json entries for the keys
for s in &words {
// add them to the CF2 cuckoo filter structure
    if CF2.test_and_add(s) {
        insertions += 1;
    }
}
// we return the CuckooFilter structure from this method and this is referred by the CF global
// variable created in lazy static method.
CF2
}
// This is the crucial method with a wasm_bindgen annotation. This is the method exposed to the host 
// and to the wasm bindgen utility to generate the glue code on js side and make changes on the wasm 
// side as well. As one can see it takes string as input and returns a Boolean wrapped in a structure . We //will expose the check_word_exists method to the host
//JSValue
#[wasm_bindgen]
pub fn check_word_exists(member:&str) ->JsValue
{
// check for presence of the keyword sent from host
let exists=cf.contains(member);
println!("{}",exists);
//wrap the true or false return into the JsValue structure and return it
JsValue::from_serde(&exists).unwrap()
}
