#!/bin/bash

outdir=$PWD/_out
filename=calculator

case "$1" in 
  bindgen)
    if [ ! -d wit/deps/adder ]; then
      echo "缺乏依赖 deps/adder :("
      echo "请先执行 renew-deps 子命令"
      exit 1
    fi

    outdir=bindings
    rm -rf $outdir
    mkdir $outdir
    
    wit-bindgen tiny-go --out-dir $outdir --rename-package bindings ./wit
  ;;

  build)
    rm -rf $outdir
    mkdir -p $outdir
    tinygo build -target=wasi -o $outdir/$filename.wasm main.go
  ;;

  componentize)
    world=`grep '^world' wit/world.wit | awk '{print $2}'`
    if [ -z "$world" ]; then
      echo "必须指明 'world'"
      exit 2
    fi

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

  renew-deps)
    rm -rf wit/deps
    mkdir -p wit/deps

    # @warn: 相对路径不行
    ln -sf $PWD/../adder/wit wit/deps/adder
  ;;

  *)
    echo "Usage: $0 {bindgen|build|componentize|renew-deps}"
    exit 1
  ;;
esac
