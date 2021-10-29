const { instantiate } = require("@wapc/host");
const { encode, decode } = require("@msgpack/msgpack");
const { promises: fs } = require("fs");
const path = require("path");

// Argument as index 0 is the node executable, index 1 is the wasm filename

const wasmfile = process.argv[2]; //wasm file as input
const operation = process.argv[3]; // function defined in wasm file (check_word_exists)
const json = process.argv[4]; //json for input parameters to the function

// If we don't have the basic arguments we need, print usage and exit.
if (!(wasmfile && operation && json)) {
  console.log("Usage: node index.js [wasm file] [waPC operation] [JSON input]");
  process.exit(1);
}

async function main() {
  // Read wasm off the local disk as Uint8Array
  buffer = await fs.readFile(path.join(__dirname, wasmfile));

  // Instantiate a WapcHost with the bytes read off disk
  const host = await instantiate(buffer);

  // Parse the input JSON and encode as msgpack
  const payload = encode(JSON.parse(json));

  // Invoke the operation in the wasm guest
  const result = await host.invoke(operation, payload);

  // Decode the results using msgpack
  const decoded = decode(result);

  // log to the console
  console.log(`Result: ${decoded}`);
}

main().catch((err) => console.error(err));
