#!/bin/bash

repo_tag=sammyne/wasm-examples-playground:634ab00

docker run -it --rm -v $PWD:/workspace -w /workspace $repo_tag bash
