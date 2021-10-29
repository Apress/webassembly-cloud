extern crate wapc;
extern crate wasmtime_provider;
extern crate wascc_codec;
extern crate serde_json;
extern crate serde_derive;
extern crate serdeconv;
use std::collections::HashMap;
use std::fs::read;
use std::env;
use wapc::WapcHost;


fn runs_wapc_guest() -> anyhow::Result<()> {

 let args: Vec<String> = env::args().collect();

    let wasmpath= &args[1]; //wasm file path. We will use the cuckoo wasm path here
    let key = &args[2]; //name of the json key
let val= &args[3]; // value

let mut scores = HashMap::new();
// create a hashmap and insert the key value pair
    scores.insert(key.to_string(),val.to_string());

// convert the key value to  message pack 
let p2=serdeconv::to_msgpack_vec(&scores).unwrap();
    let buf = read(wasmpath.to_string())?;

// This code creates the wasmtime Engine 

    let engine = wasmtime_provider::WasmtimeEngineProvider::new(&buf, None);

// We use the wapc wrapper over the wasmtime engine here.
    let guest = WapcHost::new(Box::new(engine), move |_a, _b, _c, _d, _e| Ok(vec![]))?;

// we invoke the function check_word_exists here and pass the msgpack encoded message
    let callresult = guest.call("check_word_exists", &p2).unwrap();

// response is also encoded by msgpack, so we need to get the string back from it.
let p1:String=serdeconv::from_msgpack_slice(&callresult[..]).unwrap();
println!("response{:?}",&p1);
    Ok(())
}
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
runs_wapc_guest();


    Ok(())
}
