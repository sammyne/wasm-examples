#!/bin/bash

set -e

cd `dirname ${BASH_SOURCE[0]}`

repo_tag=sammyne/wasm-examples-playground:`git rev-parse --short HEAD`

build_arg_opts="--build-arg CARGO_COMPONENT_VERSION=0.11.0"
build_arg_opts="$build_arg_opts --build-arg GO_VERSION=1.22.3"
build_arg_opts="$build_arg_opts --build-arg MOLD_VERSION=2.30.0"
build_arg_opts="$build_arg_opts --build-arg TINYGO_VERSION=0.31.2"
build_arg_opts="$build_arg_opts --build-arg WASM_TOOLS_VERSION=1.206.0"
build_arg_opts="$build_arg_opts --build-arg WASMTIME_VERSION=20.0.0"
build_arg_opts="$build_arg_opts --build-arg WIT_BINDGEN_VERSION=0.25.0"

docker build $build_args_opts -t $repo_tag .
