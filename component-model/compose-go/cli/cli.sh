#!/bin/bash

set -e

outdir=$PWD/_out
filename=cli
deps=(adder calculator)

case "$1" in 
  bindgen)
    for v in ${deps[@]}; do
      d=wit/deps/$v
      if [ ! -d $d ]; then
        echo "缺乏依赖 $d :("
        echo "请先执行 renew-deps 子命令"
        exit 1
      fi
    done

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
    wasm-tools component embed --world $world ./wit $outdir/$filename.wasm -o $outdir/$filename-embed.wasm # create a component

    cd $outdir
    wasm-tools component new $filename-embed.wasm                               \
      --adapt /opt/wasmtime/adapter-modules/wasi_snapshot_preview1.command.wasm \
      -o $filename-component.wasm
    wasm-tools validate $filename-component.wasm --features component-model
  ;;

  compose)
    for v in ${deps[@]}; do
      cp $PWD/../$v/_out/$v-component.wasm $outdir/
    done

    cd $outdir

    # @warn: compose 命令要求文件名必须用 '-' 字符分隔，不能用 '.' 分隔
    wasm-tools compose calculator-component.wasm -d adder-component.wasm -o calculator-composed.wasm
    wasm-tools compose $filename-component.wasm -d calculator-composed.wasm -o $filename-composed.wasm

    wasm-tools validate $filename-composed.wasm --features component-model
  ;;

  renew-deps)
    rm -rf wit/deps
    mkdir wit/deps

    for v in ${deps[@]}; do
      rm -rf $PWD/../$v/wit/deps
      # @warn: 相对路径不行
      ln -sf $PWD/../$v/wit wit/deps/$v
    done
  ;;

  *)
    echo "Usage: $0 {bindgen|build|componentize|renew-deps}"
    exit 1
  ;;
esac
