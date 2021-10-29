const { readFileSync } = require("fs");

const run = async () => {
  const buffer = readFileSync("./example.wasm");
  const module = await WebAssembly.compile(buffer);
  const instance = await WebAssembly.instantiate(module);
  console.log(instance.exports.hellowat2wasm());
};

run();
