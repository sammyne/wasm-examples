# ABI

## 快速开始
### 1. 编译
```bash
cd wasm
cargo build --release
cd -
```

### 2. 运行
```bash
cd cli
cargo run -- ../../../target/wasm32-wasi/release/abi.wasm
cd -
```

## 温馨提示

### 关于构建
由 https://github.com/rust-lang/rust/issues/71871 可知，在 `wasm32-unknown-unknown` 目标下，rust 编译器会把结构体字段展开平铺
到形参列表，和基于 clang 生成的 ABI 不兼容，无法和其他语言的 WASM 模块实现互操作。

基于 `wasm32-unknown-unknown` 目标生成 WASM 文件样例如下，可见 `Greeting` 结构被展开为两个 `i32` 字段，
```wasm
(module
  (type $#type0 (;0;) (func (param $#local0 i32) (param $#local1 i32) (result i32)))
  (func $pass_struct (;0;) (type $#type0) (param $#local0 i32) (param $#local1 i32) (result i32)
    i32.const 1
    i32.const 2
    local.get $#local1
    select
    local.get $#local0
    i32.add
  )
  (table $#table0 (;0;) 1 1 funcref)
  (memory $#memory0 (;0;) 16)
  (global $__stack_pointer (;0;) (mut i32) i32.const 1048576)
  (global $#global1 (;1;) i32 i32.const 1048576)
  (global $#global2 (;2;) i32 i32.const 1048576)
  (export "memory" (memory $#memory0))
  (export "pass_struct" (func $pass_struct))
  (export "__data_end" (global $#global1))
  (export "__heap_base" (global $#global2))
  (@producers
    (language "Rust" "")
    (processed-by "rustc" "1.77.1 (7cf61ebde 2024-03-27)")
  )
  (@custom "target_features" (after code) "\02+\0fmutable-globals+\08sign-ext")
)
```

这一笔提交 https://github.com/rust-lang/rust/pull/79998 为 wasi 修复了上述兼容性问题，因此可使用 wasm32-wasi 目标。

## 参考文献
- [Rust to WebAssembly the hard way](https://surma.dev/things/rust-to-webassembly/)
