use std::path::PathBuf;

use anyhow::Context as _;
use clap::Parser;
use wasmtime::component::{Component, Linker, ResourceTable, Val};
use wasmtime::{Config, Engine, Store};
use wasmtime_wasi::{WasiCtx, WasiView};

fn main() -> anyhow::Result<()> {
    let Cli { path, name } = Cli::parse();

    let mut config = Config::default();
    config.wasm_component_model(true);
    let engine = Engine::new(&config)?;
    let mut linker = Linker::new(&engine);

    // Add the command world (aka WASI CLI) to the linker
    wasmtime_wasi::add_to_linker_sync(&mut linker).context("link command world")?;

    let mut store = Store::new(&engine, State::new());

    let component = Component::from_file(&engine, path).context("Component file not found")?;

    const INTERFACE: &str = "sammyne:helloworld/greeter@1.0.0";
    const FUNC_NAME: &str = "say-hello";

    let instance = linker
        .instantiate(&mut store, &component)
        .context("instantiate")?;

    let f = instance
        .exports(&mut store)
        .instance(INTERFACE)
        .ok_or_else(|| anyhow::anyhow!("miss interface: {INTERFACE}"))?
        .func(FUNC_NAME)
        .with_context(|| format!("miss func '{FUNC_NAME}'"))?;

    let params = [new_hello_request(name.clone())];
    let mut results = [Val::Bool(false)];
    f.call(&mut store, &params, &mut results).context("call")?;
    // post-return 清理 say-hello 关联的状态。
    f.post_return(&mut store)
        .with_context(|| format!("post return '{FUNC_NAME}'"))?;
    println!("say-hello returns {results:?}");

    Ok(())
}

/// A CLI for executing WebAssembly components that
/// implement the `example` world.
#[derive(Parser)]
#[clap(name = "hello-world-host", version = env!("CARGO_PKG_VERSION"))]
struct Cli {
    /// WASM 组件的路径
    #[clap(short, long)]
    path: PathBuf,
    /// greet 函数的入参依赖的 name 字段。
    #[clap(short, long)]
    name: String,
}

struct State {
    ctx: WasiCtx,
    table: ResourceTable,
}

impl State {
    fn new() -> Self {
        Self {
            ctx: wasmtime_wasi::WasiCtxBuilder::new().build(),
            table: ResourceTable::new(),
        }
    }
}

impl WasiView for State {
    fn ctx(&mut self) -> &mut WasiCtx {
        &mut self.ctx
    }

    fn table(&mut self) -> &mut ResourceTable {
        &mut self.table
    }
}

fn new_hello_request(name: String) -> Val {
    Val::Record(vec![("name".to_owned(), Val::String(name))])
}
