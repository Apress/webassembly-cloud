const { readFileSync } = require("fs");
// this is one of the variable which will be imported by the guest
var memory = new WebAssembly.Memory({ initial : 1 });
const run = async () => {
var importObject = {
//the function and memory are defined as imports .These are imported by the wasm module.
  example: {
    log: function(offset,length) {
var bytes = new Uint8Array(memory.buffer, offset, length);
  var string = new TextDecoder('utf8').decode(bytes);
  console.log(string);
    }
  }
,js: {
          mem: memory
        }
};
  const buffer = readFileSync("./string.wasm");
  const module = await WebAssembly.compile(buffer);
  const instance = await WebAssembly.instantiate(module,importObject);
  instance.exports.logme();
};
run();
