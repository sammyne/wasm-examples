mod utils;

use std::fs;

use anyhow::Context;
use wasmtime::{Caller, Engine, Error, Extern, Linker, Module, Store};

use crate::utils::TinyMemory;

fn main() -> wasmtime::Result<()> {
    let path = std::env::args()
        .skip(1)
        .next()
        .ok_or_else(|| Error::msg("miss wasm path"))?;

    let bytecodes = fs::read(&path).context("read file")?;

    let engine = Engine::default();

    let module = Module::new(&engine, &bytecodes).context("new module")?;

    let mut linker = Linker::new(&engine);

    linker
        .func_wrap("env", "renew_greeting", renew_greeting)
        .context("wrap renew_greeting")?;

    demo_call_host_with_complex_args_and_returns(&engine, &linker, &module)
        .context("demo call_host_with_complex_args_and_returns")?;
    demo_pass_struct(&engine, &linker, &module).context("demo pass_struct")?;
    demo_return_struct(&engine, &linker, &module).context("demo return_struct")?;

    Ok(())
}

#[derive(Debug, Default)]
#[repr(C)]
struct Greeting {
    pub a: u32,
    pub b: bool,
}

/// 演示实参和返回值都为结构体
fn demo_call_host_with_complex_args_and_returns(
    engine: &Engine,
    linker: &Linker<()>,
    module: &Module,
) -> anyhow::Result<()> {
    let mut store = Store::new(&engine, ());

    let instance = linker
        .instantiate(&mut store, module)
        .context("instantiate")?;

    const INPUT_OFFSET: usize = 100;
    // @warn: 必须使用实例导出的内存模块，而且名称也需要和 wasm 文件里面到处的名称一致。
    // @warn: 另外新建内存模块，对此实例没有作用。
    let mem = instance
        .get_memory(&mut store, "memory")
        .context("get memory")?;

    let input = Greeting { a: 789, b: true };
    TinyMemory::new(&mem, &mut store)
        .write(INPUT_OFFSET, &input)
        .context("write input to memory")?;

    let f = instance
        .get_typed_func::<(i32, i32), ()>(&mut store, "call_host_with_complex_args_and_returns")
        .context("load func 'return_struct'")?;

    const OUTPUT_OFFSET: usize = INPUT_OFFSET + 100;
    // todo: 测试大小端是否影响结果正确性
    f.call(&mut store, (OUTPUT_OFFSET as i32, INPUT_OFFSET as i32))
        .context("call_host_with_complex_args_and_returns.call")?;

    let output: Greeting = TinyMemory::new(&mem, &mut store)
        .read(OUTPUT_OFFSET)
        .context("read output from memory")?;

    assert_eq!(889, output.a, "bad a");
    assert!(!output.b, "bad b");

    Ok(())
}

/// 演示实参是结构体
fn demo_pass_struct(engine: &Engine, linker: &Linker<()>, module: &Module) -> anyhow::Result<()> {
    let mut store = Store::new(&engine, ());

    let instance = linker
        .instantiate(&mut store, module)
        .context("instantiate")?;

    const GREETING_OFFSET: usize = 200;
    // @warn: 必须使用实例导出的内存模块，而且名称也需要和 wasm 文件里面到处的名称一致。
    // @warn: 另外新建内存模块，对此实例没有作用。
    let mem = instance
        .get_memory(&mut store, "memory")
        .context("get memory")?;
    let greeting = Greeting { a: 123, b: true };
    TinyMemory::new(&mem, &mut store)
        .write(GREETING_OFFSET, &greeting)
        .context("write greeting")?;

    let pass_struct = instance
        .get_typed_func::<i32, i32>(&mut store, "pass_struct")
        .context("load func 'pass_struct'")?;

    // ref: https://github.com/WebAssembly/tool-conventions/blob/main/BasicCABI.md#function-arguments-and-return-values
    // ref: https://github.com/rust-lang/rust/issues/71871 -> rust 编译器会把结构体字段展开平铺到形参列表，而 clang 不会
    let out = pass_struct
        .call(&mut store, GREETING_OFFSET as i32)
        .context("pass_struct.call")?;

    assert_eq!(124, out, "bad out");

    Ok(())
}

/// 演示返回值是 1 个结构体
fn demo_return_struct(engine: &Engine, linker: &Linker<()>, module: &Module) -> anyhow::Result<()> {
    let mut store = Store::new(&engine, ());

    let instance = linker
        .instantiate(&mut store, module)
        .context("instantiate")?;

    const GREETING_OFFSET: usize = 100;

    let f = instance
        .get_typed_func::<(i32, i32, i32), ()>(&mut store, "return_struct")
        .context("load func 'return_struct'")?;

    // todo: 测试大小端是否影响结果正确性
    f.call(&mut store, (GREETING_OFFSET as i32, 123, true as i32))
        .context("return_struct.call")?;

    // @warn: 必须使用实例导出的内存模块，而且名称也需要和 wasm 文件里面到处的名称一致。
    let mem = instance
        .get_memory(&mut store, "memory")
        .context("get memory")?;

    let out: Greeting = TinyMemory::new(&mem, &mut store)
        .read(GREETING_OFFSET)
        .context("read output from memory")?;

    assert_eq!(123, out.a, "bad a");
    assert!(out.b, "bad b");

    Ok(())
}

fn renew_greeting(caller: Caller<'_, ()>, out_ptr: i32, in_ptr: i32) {
    let mut caller = caller;
    let mem = match caller.get_export("memory").expect("miss exported memory") {
        Extern::Memory(v) => v,
        _ => panic!("exported section 'memory' isn't memory"),
    };

    let mut out = TinyMemory::new(&mem, &mut caller)
        .read::<Greeting>(in_ptr as usize)
        .expect("read memory");

    out.a += 100;
    out.b = !out.b;

    TinyMemory::new(&mem, &mut caller)
        .write(out_ptr as usize, &out)
        .expect("write output")
}
