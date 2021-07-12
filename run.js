'use strict';

const fs = require('fs');
const path = require('path');
const NAPI = require('./napi');
const { WASI } = require('wasi');

const bin = fs.readFileSync(process.argv[2]);

const mod = new WebAssembly.Module(bin);

const napi = new NAPI();
const wasi = new WASI({
  argv: ['run'],
  preopens: {
    '/sandbox': path.resolve('./sandbox'),
  },
  returnOnExit: true,
});
const instance = new WebAssembly.Instance(mod, {
  env: { main() {} },
  napi: napi.exports,
  wasi_snapshot_preview1: wasi.wasiImport,
});

wasi.initialize(instance);

const e = napi.init(instance);

/*
function test(binding) {
  const start = Date.now();
  let count = 0;
  for (let i = 0; i < 1000000; i += 1) {
    binding.call(() => {
      count += 1;
    });
  }
  const end = Date.now();
  console.log(count, end - start);
  return end - start;
}

test(require('./test.node'));
test(require('./test.node'));
const native = test(require('./test.node'));
test(e);
test(e);
const wasm = test(e);
console.log(native / wasm);
*/

console.log('instance.readFile():', e.readFile(1, 2, 3));
