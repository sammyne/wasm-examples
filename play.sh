#!/bin/bash

repo_tag=sammyne/wasm-examples-playground:149588b

docker run -it --rm -v $PWD:/workspace -w /workspace $repo_tag bash
