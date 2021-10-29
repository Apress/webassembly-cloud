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
    let listener = TcpListener::bind("0.0.0.0:8080").unwrap();

 let args: Vec<String> = env::args().collect();

    let wasmpath= &args[1];
    //let key = &args[2];
//let val= &args[3];

    let some_state = "state".to_string();
let guest =   wasm::engine_start(wasmpath.to_string()).unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream,&guest);
    }
}

fn handle_connection(mut stream: TcpStream,guest:&wapc::WapcHost) {
    let mut buffer = [0; 512];

    stream.read(&mut buffer).unwrap();
//let request = String::from_utf8_lossy(&buffer);
  //          println!("{}", request);

let req: HttpRequest = String::from_utf8(buffer.to_vec()).unwrap().into();

let body= &req.msg_body;

println!("body={}",body);


//let request_line = request.lines().next().unwrap();
//let mut parts = request_line.split_whitespace();

/*
match parse_request_line(&request_line) {
   Ok(request) => {
            info!("\n{}", request);
        }
        Err(()) => error!("Badly formatted request: {}", &request_line),
    }*/

let p1=wasm::runs_wapc_guest(&guest,body).unwrap();
//println!("result {}",p1);
// let response = "HTTP/1.1 200 OK\r\n\r\n";
let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", p1);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
//    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
}

