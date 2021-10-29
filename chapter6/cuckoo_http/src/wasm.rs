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
	use std::error::Error;
	use wasmtime_provider::WasmtimeEngineProvider;
//use std::collections::HashMap;
// this method accepts the path of the wasm file as input. This starts the engine and loads the //wasm module.
	pub fn engine_start(wasmpath:String) -> Result<WapcHost, Box<dyn Error>>
	{
	let args: Vec<String> = env::args().collect();

	let buf = read(wasmpath.to_string())?;

	let engine = wasmtime_provider::WasmtimeEngineProvider::new(&buf, None);
	let guest = WapcHost::new(Box::new(engine), move |_a, _b, _c, _d, _e| Ok(vec![]))?;
	Ok(guest)

	}

// this method gets the request body (key value pair). It creates a hashmap entry for the key //value
// encodes it via msgpack and then passes the encoded message to the wasm module

	pub fn runs_wapc_guest(guest:&WapcHost,body:&String) -> Result<String, Box<dyn Error>> {
//let guest = WapcHost::new(Box::new(*engine), move |_a, _b, _c, _d, _e| Ok(vec![]))?;
	let res: Vec<String> = body.split("=").map(|s| s.to_string()).collect();
//let mut scores = one_liner(body);
	let mut scores = HashMap::new();
	let x=&res[0];
	let y=&res[1];
	scores.insert(x.trim_matches(char::from(0)).to_string(),y.trim_matches(char::from(0)).to_string());

	println!("key={}",&res[0]);
	println!("value={}",&res[1]);
	println!("map{:?}",&scores);
//msgpack encoding is done here
	let p2=serdeconv::to_msgpack_vec(&scores).unwrap();
// invoke the check_word_exists method on wasm module and pass the encoded message
	let callresult = &guest.call("check_word_exists", &p2).unwrap();

// decode the msgpack encoded response back to string
	let p1:String=serdeconv::from_msgpack_slice(&callresult[..]).unwrap();
	println!("response{:?}",&p1);
	Ok(p1)
	}
