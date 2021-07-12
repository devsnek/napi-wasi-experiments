#!/bin/bash

set -ex

node-dev \
  --no-warnings \
  --harmony-weak-refs \
  --experimental-wasi-unstable-preview1 \
  --experimental-wasm-bigint \
  run.js ./build/wasi_test.wasm
