# 动态构建组件依赖的实例

## 快速开始

### 1. 编译
```bash
cd component
cargo component build --release --lib
cd -
```

### 2. 运行

```bash
cd cli
# hello_world.wasm 是上一步的编译产物
cargo run -- ../../../target/wasm32-wasi/release/dynamically_built.wasm
cd -
```

## 参考文献
- https://component-model.bytecodealliance.org/language-support/rust.html
