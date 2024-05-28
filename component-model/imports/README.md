# 模块组合

## 1. 构建
### 1.1. 构建组件 1
```bash
cd component1
cargo component build --release
cd -
```

### 1.2. 构建组件 2
```bash
cd component2
cargo component build --release
cd -
```

### 1.3. 构建 cli 组件
```bash
cd cli
cargo component build --release
cd -
```

## 2. 组合

```bash
cd ../../target/wasm32-wasi/release

# @warn: wasm-tools 要求文件名必须为 kecab-case 形式。
mv imports_component1.wasm imports-component1.wasm
mv imports_component2.wasm imports-component2.wasm

# 组合 adder 和 calculator 模块
wasm-tools compose imports-component2.wasm -d imports-component1.wasm -o imports-component2-composed.wasm

wasm-tools compose imports-cli.wasm -d imports-component2-composed.wasm -o imports-cli-composed.wasm
```

> 温馨提示：wasm-tools 尚不支持间接依赖

## 3. 运行

```bash
wasmtime run imports-cli-composed.wasm
```

## 参考文献
- [The WebAssembly Component Model/Components in Rust](https://component-model.bytecodealliance.org/language-support/rust.html)
