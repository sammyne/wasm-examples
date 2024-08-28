#!/bin/bash

cd `dirname ${BASH_SOURCE[0]}`

case "$1" in 
  bindgen)
    outdir=bindings
    rm -rf $outdir
    mkdir $outdir

    rm -rf wit
    cp -r ../component/wit wit
    mkdir wit/deps
    cp -r ../cli/wit wit/deps/cli
    
    # wit-bindgen 假设组件的依赖 wit 放在相同目录的 deps 目录下。
    # 具体要求参见 https://docs.rs/wit-parser/latest/wit_parser/struct.Resolve.html#method.push_path
    wit-bindgen tiny-go --out-dir $outdir --rename-package bindings ./wit
  ;;

  build)
    rm -rf out
    mkdir -p out
    tinygo build -target=wasi -o out/hello-world.wasm main.go
  ;;

  componentize)
    filename=hello-world

    world=`grep '^world' wit/world.wit | awk '{print $2}'`

    # 将一个 wasm 模块包装为一个组件
    # @note: --world 选项指定的值必须和 wit 指定的 world 匹配。
    wasm-tools component embed --world $world ./wit out/$filename.wasm -o out/$filename.embed.wasm # create a component

    cd out
    wasm-tools component new $filename.embed.wasm                               \
      --adapt /opt/wasmtime/adapter-modules/wasi_snapshot_preview1.reactor.wasm \
      -o $filename.component.wasm
    wasm-tools validate $filename.component.wasm --features component-model
  ;;

  *)
    echo "Usage: $0 {bindgen|build|componentize}"
    exit 1
  ;;
esac
