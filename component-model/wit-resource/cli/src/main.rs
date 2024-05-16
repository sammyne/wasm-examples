use std::path::PathBuf;

use anyhow::{anyhow, Context};
use clap::Parser;
use wasmtime::component::*;
use wasmtime::{Config, Engine, Store};
use wasmtime_wasi::preview1::WasiP1Ctx;
use wasmtime_wasi::{WasiCtx, WasiCtxBuilder, WasiView};

use bindings::helloworld::cli::api::{self, Host, HostGreeter};

mod bindings {
    wasmtime::component::bindgen!({});
}

fn main() -> anyhow::Result<()> {
    let App { path } = App::parse();

    let mut config = Config::default();
    config.wasm_component_model(true);
    let engine = Engine::new(&config)?;
    let mut linker = Linker::new(&engine);

    // Add the command world (aka WASI CLI) to the linker
    wasmtime_wasi::add_to_linker_sync(&mut linker).context("link command world")?;
    api::add_to_linker(&mut linker, |v: &mut TinyHostRegistry| &mut v.greeter)
        .context("add TinyHostRegistry to linker")?;

    let wasi_view = TinyHostRegistry::new();
    let mut store = Store::new(&engine, wasi_view);

    let component = Component::from_file(&engine, path).context("Component file not found")?;

    let instance = linker
        .instantiate(&mut store, &component)
        .context("Failed to instantiate the example world")?;

    let f = instance
        .get_func(&mut store, "hello-world")
        .context("get func 'hello-world'")?;

    let mut returns = vec![Val::Bool(false)];
    f.call(&mut store, &[], &mut returns).context("call")?;

    let msg = match returns.pop().expect("miss returned value") {
        Val::String(s) => s,
        v => return Err(anyhow!("unexpected return types: {v:?}")),
    };
    println!("{msg}");

    Ok(())
}

/// A CLI for executing WebAssembly components that
/// implement the `example` world.
#[derive(Parser)]
#[clap(name = "hello-world-host", version = env!("CARGO_PKG_VERSION"))]
struct App {
    /// The path to the component.
    #[clap(value_name = "COMPONENT_PATH")]
    path: PathBuf,
}

struct Greeter;

struct TinyHostRegistry {
    greeter: Greeter,
    ctx: WasiP1Ctx,
}

impl HostGreeter for Greeter {
    fn new(&mut self) -> wasmtime::Result<Resource<api::Greeter>> {
        let out = Resource::new_own(123);
        Ok(out)
    }

    fn greet(&mut self, _self_: Resource<api::Greeter>, who: String) -> wasmtime::Result<()> {
        println!("hello {who} :)");
        Ok(())
    }

    fn drop(&mut self, _rep: Resource<api::Greeter>) -> wasmtime::Result<()> {
        Ok(())
    }
}

impl Host for Greeter {}

impl TinyHostRegistry {
    fn new() -> Self {
        Self {
            greeter: Greeter,
            ctx: WasiCtxBuilder::new().inherit_stdout().build_p1(),
        }
    }
}

impl WasiView for TinyHostRegistry {
    fn table(&mut self) -> &mut ResourceTable {
        self.ctx.table()
    }

    fn ctx(&mut self) -> &mut WasiCtx {
        self.ctx.ctx()
    }
}
