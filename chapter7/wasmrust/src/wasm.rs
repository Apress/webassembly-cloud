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
pub fn engine_start(wasmpath:String) -> Result<WapcHost, Box<dyn Error>>
{
 let args: Vec<String> = env::args().collect();

//    let wasmpath= &args[1];
//    let key = &args[2];
//let val= &args[3];

//let mut scores = HashMap::new();

 //   scores.insert(key.to_string(),val.to_string());


//let p2=serdeconv::to_msgpack_vec(&scores).unwrap();
    let buf = read(wasmpath.to_string())?;

    let engine = wasmtime_provider::WasmtimeEngineProvider::new(&buf, None);
    let guest = WapcHost::new(Box::new(engine), move |_a, _b, _c, _d, _e| Ok(vec![]))?;
Ok(guest)

}

/*fn one_liner(string: &str) -> HashMap<&str, &str> {
let res: Vec<String> = str.split("=").map(|s| s.to_string()).collect();
//let (key,val)=str.split_at(str.find("=").unwrap()); 
let mut scores = HashMap::new();
scores.insert(res[0],res[1]);
scores
//    string.split_whitespace().map(|s| s.split_at(s.find("=").unwrap())).map(|(key, val)| (key, &val[1..])).collect()
}*/

pub fn runs_wapc_guest(guest:&WapcHost,body:&String) -> Result<String, Box<dyn Error>> {
//let guest = WapcHost::new(Box::new(*engine), move |_a, _b, _c, _d, _e| Ok(vec![]))?;
/* let args: Vec<String> = env::args().collect();

    let wasmpath= &args[1];
    let key = &args[2];
let val= &args[3];
*/
let res: Vec<String> = body.split("=").map(|s| s.to_string()).collect();
//let mut scores = one_liner(body);
let mut scores = HashMap::new();
if(res.len()>=2)
{
let x=&res[0];
let y=&res[1];
scores.insert(x.trim_matches(char::from(0)).to_string(),y.trim_matches(char::from(0)).to_string());

println!("key={}",&res[0]);
println!("value={}",&res[1]);
//scores.insert(&res[0],&res[1]);

//
//    scores.insert("name","foo");
println!("map{:?}",&scores);

let p2=serdeconv::to_msgpack_vec(&scores).unwrap();
/*
    let buf = read(wasmpath.to_string())?;

    let engine = wasmtime_provider::WasmtimeEngineProvider::new(&buf, None);
    let guest = WapcHost::new(Box::new(engine), move |_a, _b, _c, _d, _e| Ok(vec![]))?;

*/
    let callresult = &guest.call("check_word_exists", &p2).unwrap();


let p1:String=serdeconv::from_msgpack_slice(&callresult[..]).unwrap();

println!("response{:?}",&p1);
    Ok(p1)
}
else
{
Ok("error".to_string())
}
}
pub fn launch() -> Result<(), Box<dyn std::error::Error>> {
//runs_wapc_guest();


    Ok(())
}

