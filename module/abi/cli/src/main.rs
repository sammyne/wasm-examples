use core::slice;
use std::fs;

use anyhow::Context;
use wasmtime::{Engine, Error, Linker, Module, Store};

fn main() -> wasmtime::Result<()> {
    let path = std::env::args()
        .skip(1)
        .next()
        .ok_or_else(|| Error::msg("miss wasm path"))?;

    let bytecodes = fs::read(&path).context("read file")?;

    let engine = Engine::default();

    let module = Module::new(&engine, &bytecodes).context("new module")?;

    let linker = Linker::new(&engine);

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

fn demo_pass_struct(engine: &Engine, linker: &Linker<()>, module: &Module) -> anyhow::Result<()> {
    let mut store = Store::new(&engine, ());

    let instance = linker
        .instantiate(&mut store, module)
        .context("instantiate")?;

    const GREETING_OFFSET: usize = 200;
    const GREETING_LENGTH: usize = std::mem::size_of::<Greeting>();
    {
        // @warn: 必须使用实例导出的内存模块，而且名称也需要和 wasm 文件里面到处的名称一致。
        // @warn: 另外新建内存模块，对此实例没有作用。
        let mem = instance
            .get_memory(&mut store, "memory")
            .context("get memory")?;

        let greeting = Greeting { a: 123, b: true };
        let data = unsafe {
            let data = &greeting as *const Greeting as *const u8;
            slice::from_raw_parts(data, GREETING_LENGTH)
        };

        //let buf = mem.data_mut(&mut store);
        //buf[GREETING_OFFSET..(GREETING_OFFSET + data.len())].copy_from_slice(data);
        mem.write(&mut store, GREETING_OFFSET, data)
            .context("write greeting to memory")?;
    }

    let pass_struct = instance
        .get_typed_func::<i32, i32>(&mut store, "pass_struct")
        .context("load func 'pass_struct'")?;

    // ref: https://github.com/WebAssembly/tool-conventions/blob/main/BasicCABI.md#function-arguments-and-return-values
    // ref: https://github.com/rust-lang/rust/issues/71871 -> rust 编译器会把结构体字段展开平铺到形参列表，而 clang 不会
    let out = pass_struct
        .call(&mut store, GREETING_OFFSET as i32)
        .context("pass_struct.call")?;
    println!("out = {out}");

    Ok(())
}

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

    let out = {
        // @warn: 必须使用实例导出的内存模块，而且名称也需要和 wasm 文件里面到处的名称一致。
        let mem = instance
            .get_memory(&mut store, "memory")
            .context("get memory")?;

        let mut greeting = Greeting::default();
        let data = unsafe {
            let data = &mut greeting as *mut Greeting as *mut u8;
            slice::from_raw_parts_mut(data, std::mem::size_of::<Greeting>())
        };

        mem.read(&mut store, GREETING_OFFSET, data)
            .context("read greeting from memory")?;

        greeting
    };

    println!("out = {out:?}");

    Ok(())
}
