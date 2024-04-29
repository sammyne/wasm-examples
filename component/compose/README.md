# 模块组合

```bash
cd ../../target/wasm32-wasi/release

wasm-tools compose calculator.wasm -d adder.wasm -o calculator-composed.wasm

# 由于 wasm-tools 尚不支持简介依赖，所以下述命令不行
wasm-tools compose cli.wasm -d adder.wasm -d calculator.wasm -o cli-composed.wasm
# 而以下命令可以
wasm-tools compose cli.wasm -d calculator-composed.wasm -o cli-composed.wasm

cd -
```
