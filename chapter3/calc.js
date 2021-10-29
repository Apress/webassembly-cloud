const { readFileSync } = require("fs");

const run = async () => {
  const buffer = readFileSync("./calc.wasm");
  const module = await WebAssembly.compile(buffer);
  const instance = await WebAssembly.instantiate(module);
var sum=  instance.exports.add(34,76);
var diff=instance.exports.subtract(76,34);
var mul=instance.exports.multiply(12,8);
console.log("sum of 34 and 76="+sum);
console.log("difference of 76 and 34="+diff);
console.log("product of 12 and 8="+mul);
};

run();
