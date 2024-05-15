#!/bin/bash

repo_tag=sammyne/wasm-examples-playground:149588b

docker run -it --rm                             \
  -e CARGO_HOME=/root/.cargo                    \
  -v $PWD:/workspace                            \
  -v $PWD/_cargo/registry:/root/.cargo/registry \
  -v $PWD/_cargo/git:/root/.cargo/git           \
  -w /workspace                                 \
  $repo_tag                                     \
    bash
