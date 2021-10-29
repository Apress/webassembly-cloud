extern crate wasmtime_provider;
pub mod httprequest;
use std::io::prelude::*;
use httprequest::HttpRequest;
use std::net::TcpListener;
use std::net::TcpStream;
use std::env;
extern crate wapc;
use wasmtime_provider::WasmtimeEngineProvider;
pub mod wasm;
fn main() {
// create a TCP listener to listen on localhost at port 8080
	let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

	let args: Vec<String> = env::args().collect();
// pass the wasm file path as an argument to the program binary
	let wasmpath= &args[1];
// launch the wasm engine with loading the wasm module at start.
	let guest =   wasm::engine_start(wasmpath.to_string()).unwrap();
// start listening for TCP connections here
	for stream in listener.incoming() {
	let stream = stream.unwrap();
// Handle the TCP request by invoking this method
	handle_connection(stream,&guest);
	}
	}
// connection Handler for the TCP connection
	fn handle_connection(mut stream: TcpStream,guest:&wapc::WapcHost) {
	let mut buffer = [0; 512];

	stream.read(&mut buffer).unwrap();
//create an http request from the tcp request.
	let req: HttpRequest = String::from_utf8(buffer.to_vec()).unwrap().into();

	let body= &req.msg_body;

	println!("body={}",body);


// invoke the method on wasm.rs file and pass the http request body (key value pair) to it.
	let p1=wasm::runs_wapc_guest(&guest,body).unwrap();
	let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", p1);
// write the response back
	stream.write(response.as_bytes()).unwrap();
	stream.flush().unwrap();
	}
