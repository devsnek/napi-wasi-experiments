#!/bin/bash

set -ex

if [ ${1: -3} == ".rs" ]
then
  rustc $1 \
    -O \
    --target wasm32-wasi \
    -C link-arg=--export-table \
    -o build/wasi_test.wasm
else
  ROOT=$HOME/Desktop/tools/wasi-sdk/build/install/opt/wasi-sdk

  NODE_INCLUDE_DIR=$HOME/.cache/node-gyp/$(node -p "process.version.slice(1)")/include/node
  NAPI_CC_INCLUDE_DIR=$(node -p "JSON.parse(require('node-addon-api').include)")

  $ROOT/bin/clang++ \
    --sysroot=$ROOT/share/wasi-sysroot \
    -target wasm32-unknown-wasi \
    -Ofast \
    -fno-exceptions \
    -fvisibility=hidden \
    -mexec-model=reactor \
    -Wl,-error-limit=0 \
    -Wl,-O3 \
    -Wl,--lto-O3 \
    -Wl,--strip-all \
    -Wl,--allow-undefined \
    -Wl,--export-dynamic \
    -Wl,--export-table \
    -DNODE_GYP_MODULE_NAME=test_wasi \
    -DNAPI_DISABLE_CPP_EXCEPTIONS \
    -I$NODE_INCLUDE_DIR \
    -I$NAPI_CC_INCLUDE_DIR \
    $1 -o build/wasi_test.wasm
fi
