#!/bin/bash

case "$1" in 
  bindgen)
    outdir=bindings
    rm -rf $outdir
    mkdir $outdir
    
    wit-bindgen tiny-go --out-dir $outdir --rename-package bindings ./wit
  ;;

  build)
    rm -rf out
    mkdir -p out
    tinygo build -target=wasi -o out/hello-world.wasm main.go
  ;;

  componentize)
    filename=hello-world

    # 将一个 wasm 模块包装为一个组件
    # @note: --world 选项指定的值必须和 wit 指定的 world 匹配。
    wasm-tools component embed --world example ./wit out/$filename.wasm -o out/$filename.embed.wasm # create a component

    cd out
    wasm-tools component new $filename.embed.wasm \
      --adapt /opt/wasmtime/adapter-modules/wasi_snapshot_preview1.reactor.wasm \
      -o $filename.component.wasm
    wasm-tools validate $filename.component.wasm --features component-model
  ;;

  *)
    echo "Usage: $0 {bindgen|build|componentize}"
    exit 1
  ;;
esac
