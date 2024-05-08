# 自定义小节

## 快速开始
### 1. 编译
```bash
cargo build --release
```

### 2. 查看字节码

```bash
wasm-tools print /workspace/target/wasm32-unknown-unknown/release/custom_section.wasm
```

样例输出如下
```bash
(module
  (table (;0;) 1 1 funcref)
  (memory (;0;) 16)
  (global $__stack_pointer (;0;) (mut i32) i32.const 1048576)
  (global (;1;) i32 i32.const 1048576)
  (global (;2;) i32 i32.const 1048576)
  (export "memory" (memory 0))
  (export "__data_end" (global 1))
  (export "__heap_base" (global 2))
  (@custom "surmsection" (after export) "hello world")
  (@producers
    (language "Rust" "")
    (processed-by "rustc" "1.77.1 (7cf61ebde 2024-03-27)")
  )
  (@custom "target_features" (after export) "\02+\0fmutable-globals+\08sign-ext")
)
```
