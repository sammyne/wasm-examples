# hello-world

## 快速开始

### rust 语言版
#### 1. 编译
```bash
cd component
cargo component build --release --lib
cd -
```

#### 2. 运行

```bash
cd cli
# hello_world.wasm 是上一步的编译产物
cargo run -- ../../../target/wasm32-wasi/release/hello_world.wasm
cd -
```

### go 语言版
#### 1. 编译
```bash
cd component-go

# 生成衔接代码
bash cli.sh bindgen

# 编译生成 wasm 模块代码
bash cli.sh build

# 模块组件化
bash cli.sh componentize

cd -
```

#### 2. 运行
```bash
cd cli
# hello_world.wasm 是上一步的编译产物
cargo run -- ../component-go/out/hello-world.component.wasm
cd -
```

## 参考文献
- https://component-model.bytecodealliance.org/language-support/rust.html
- https://tinygo.org/docs/guides/webassembly/wasi/
