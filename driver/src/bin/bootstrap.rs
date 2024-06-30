use std::sync::Arc;

use driver::module::Modules;
use driver::startup::startup;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let modules = Modules::new().await;
    let _ = startup(Arc::new(modules)).await;

    Ok(())
}
