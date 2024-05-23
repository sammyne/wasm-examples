#!/bin/bash

repo_tag=sammyne/wasm-examples-playground:15f8918

name=github-wasm-examples-playground

docker stop $name

docker run -td --rm                             \
  -e CARGO_HOME=/root/.cargo                    \
  --name $name                                  \
  -v $PWD:/workspace                            \
  -v $PWD/_cargo/registry:/root/.cargo/registry \
  -v $PWD/_cargo/git:/root/.cargo/git           \
  -w /workspace                                 \
  $repo_tag                                     \
    bash -c 'echo "export PATH=/root/.cargo/bin:$PATH" >> /root/.bashrc; bash'

if [ -f _git/gitconfig ]; then
  docker cp _git/gitconfig $name:/root/.gitconfig
fi
