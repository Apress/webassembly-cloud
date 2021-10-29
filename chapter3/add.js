const { readFileSync } = require("fs");

const run = async () => {
  const buffer = readFileSync("./add.wasm");
  const module = await WebAssembly.compile(buffer);
  const instance = await WebAssembly.instantiate(module);
  console.log(instance.exports.add(34,76));
};
run();