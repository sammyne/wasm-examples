#!/bin/bash

outdir=$PWD/_out

case "$1" in 
  bindgen)
    outdir=bindings
    rm -rf $outdir
    mkdir $outdir
    
    wit-bindgen tiny-go --out-dir $outdir --rename-package bindings ./wit
  ;;

  build)
    rm -rf $outdir
    mkdir -p $outdir
    tinygo build -target=wasi -o $outdir/adder.wasm main.go
  ;;

  componentize)
    filename=adder
    world=adder

    # 将一个 wasm 模块包装为一个组件
    # @note: --world 选项指定的值必须和 wit 指定的 world 匹配。
    wasm-tools component embed --world $world ./wit $outdir/$filename.wasm -o $outdir/$filename-embed.wasm

    cd $outdir
    # @warn: 后续在 cli 项目的组合命令要求文件名必须用 '-' 字符分隔，不能用 '.' 分隔
    wasm-tools component new $filename-embed.wasm                               \
      --adapt /opt/wasmtime/adapter-modules/wasi_snapshot_preview1.reactor.wasm \
      -o $filename-component.wasm
    wasm-tools validate $filename-component.wasm --features component-model
  ;;

  *)
    echo "Usage: $0 {bindgen|build|componentize}"
    exit 1
  ;;
esac
