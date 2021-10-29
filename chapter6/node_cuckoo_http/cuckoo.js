	const { instantiate } = require("@wapc/host");
	const { encode, decode } = require("@msgpack/msgpack");
	const { promises: fs } = require("fs");
	const http = require('http');
	const path = require("path");
	const url = require("url");
// Argument as index 0 is the node executable
// index 1 is the path to wasm file name
	const wasmfile = process.argv[2];
// index 2 is the name of operation (in our case check_word_exists)
const operation = process.argv[3];
	var host=null;
	var buffer=null;
// If we don't have the basic arguments we need, print usage and exit.
	if (!(wasmfile && operation )) {
	console.log("Usage: node index.js [wasm file] [waPC operation]");
	process.exit(1);
	}

// start of main function
	async function main() {
	console.log("entered main");
// Read wasm off the local disk as Uint8Array
	buffer = await fs.readFile(path.join(__dirname, wasmfile));

// Instantiate a WapcHost with the bytes read off disk
	host = await instantiate(buffer);
	console.log("host initiated");
	const server = http.createServer(requestListener);
	console.log("server created and listening for tcp connections on port 8080");
	server.listen(8080);
	}
// http request listener
	const requestListener = async function (req, res) {

// extract the query parameter from the http request
	let query = url.parse(req.url, true).query;
	console.log(query.key);
// get the value of the parameter key (value for this key is the json)
	const payload = encode(JSON.parse(query.key));

// Invoke the operation in the wasm guest
	const result = await   host.invoke(operation, payload);
// Decode the results using msgpack
	const decoded = decode(result);

// log to the console
	console.log(`Result: ${decoded}`);
	res.writeHead(200);
	res.end('result='+decoded);
	}

	main().catch((err) => console.error(err));
