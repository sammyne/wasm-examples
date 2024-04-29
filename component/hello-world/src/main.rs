mod helloworld;

use clap::Parser;
use std::path::PathBuf;

/// A CLI for executing WebAssembly components that
/// implement the `example` world.
#[derive(Parser)]
#[clap(name = "hello-world-host", version = env!("CARGO_PKG_VERSION"))]
struct App {
    /// The path to the component.
    #[clap(value_name = "COMPONENT_PATH")]
    component: PathBuf,
}

impl App {
    async fn run(self) -> anyhow::Result<()> {
        let msg = helloworld::hello_world(self.component).await?;
        println!("{msg}");
        Ok(())
    }
}

#[async_std::main]
async fn main() -> anyhow::Result<()> {
    App::parse().run().await

}
