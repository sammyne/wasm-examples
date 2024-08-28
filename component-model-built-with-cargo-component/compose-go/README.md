# 模块组合（Go 语言版）

## 1. 构建
### 1.1. 构建 adder 组件
```bash
cd adder

# 生成桩代码
bash cli.sh bindgen
# 编译生成模块代码
bash cli.sh build
# 组件化
bash cli.sh componentize

cd -
```

### 1.2. 构建 calculator 组件
```bash
cd calculator

# 配置好依赖到依赖项
bash cli.sh renew-deps
# 生成桩代码
bash cli.sh bindgen
# 编译生成模块代码
bash cli.sh build
# 组件化
bash cli.sh componentize

cd -
```

### 1.3. 构建 cli 应用
```bash
cd cli

# 配置好依赖到依赖项
bash cli.sh renew-deps
# 生成桩代码
bash cli.sh bindgen
# 编译生成模块代码
bash cli.sh build
# 组件化
bash cli.sh componentize
# 组合多个组件称为一个标准的 CLI 应用
bash cli.sh compose

cd -
```

> 温馨提示：wasm-tools 尚不支持间接依赖

## 3. 运行

```bash
wasmtime run cli/_out/cli-composed.wasm
```

## 参考文献
- [The WebAssembly Component Model/Components in Rust](https://component-model.bytecodealliance.org/language-support/rust.html)
