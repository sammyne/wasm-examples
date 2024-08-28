#!/bin/bash

case "$1" in 
  build)
    cargo build --release
  ;;

  componentize)
    filename=hello-world-by-wit-bindgen

    cd ../../../target/wasm32-wasi/release

    wasm-tools component new ${filename//-/_}.wasm                                     \
      --adapt /opt/wasmtime/adapter-modules/wasi_snapshot_preview1.reactor.wasm \
      -o $filename.component.wasm

    wasm-tools validate $filename-component.wasm --features component-model
  ;;

  *)
    echo "Usage: $0 {build|componentize}"
    exit 1
  ;;
esac
