const { readFileSync } = require("fs");

const run = async () => {
  const buffer = readFileSync("./table.wasm");
  const module = await WebAssembly.compile(buffer);
  const instance = await WebAssembly.instantiate(module);
var sum=  instance.exports.callByIndex(0,56,34);
var diff=instance.exports.callByIndex(1,56,34);
var mul=instance.exports.callByIndex(2,12,8);
console.log("sum of 34 and 56="+sum);
console.log("difference of 56 and 34="+diff);
console.log("product of 12 and 8="+mul);
};
run();
