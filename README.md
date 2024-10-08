# WASM 示例程序

## 快速开始

### 1. 打包开发镜像
```bash
bash docker/dockerize.sh
```

假设所得镜像为 sammyne/wasm-examples-playground:c7fa99f，后续记为 `repo_tag`。

### 2. 启动开放容器

```bash
docker run -it --rm -v $PWD:/workspace -w /workspace $repo_tag bash
```

后续在容器内借助 `cargo component` 等命令编译运行示例程序即可。

## 示例程序

目录 | 说明
----|----
component-model-built-with-cargo-component | 基于 cargo-component 工具构建的 Component Model 相关示例程序
component-model-built-with-wit-bindgen | 基于 wit-bindgen 的 `generate` 宏构建的 Component Model 相关示例程序
module | 简单的示例程序

## 温馨提示
### 关于构建
- WASM 的库和运行 WASM 的可执行文件不能放在同一个 crate，否则会由于 cargo 不区分两者的依赖，将可执行文件的依赖按 `wasm32-wasi`
目标编译触发编译错误。

## 参考文献
- [The WebAssembly Component Model](https://component-model.bytecodealliance.org/introduction.html)
