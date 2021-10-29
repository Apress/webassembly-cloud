const { readFileSync } = require("fs");

const run = async () => {
var importObject = {
  example: {
    add: function(arg1,arg2) {
     sum=arg1+arg2;
      console.log("sum="+sum);
    }
  }
};
  const buffer = readFileSync("./export.wasm");
  const module = await WebAssembly.compile(buffer);
  const instance = await WebAssembly.instantiate(module,importObject);
  instance.exports.add1();
};

run();
