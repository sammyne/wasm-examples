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

    let mut store = Store::new(&engine, 123u32);

    let instance = linker
        .instantiate(&mut store, &module)
        .context("instantiate")?;

    let add = instance
        .get_typed_func::<(u32, u32), u32>(&mut store, "add")
        .context("load func")?;

    let a = 1;
    let b = 2;

    let sum = add.call(&mut store, (a, b)).context("add.call")?;
    println!("{a} + {b} = {sum}");

    Ok(())
}
