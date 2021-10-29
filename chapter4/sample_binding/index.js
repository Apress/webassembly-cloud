const wasm = import('./sample_binding_demo');
wasm
    .then(m => m.greetMe("Shashank"))
    .catch(console.error);
