// runtime/node_agent/src/main.rs

use anyhow::Result;
use node_agent::runtime::Runtime;
use tracing::{error, info};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize structured logging
    tracing_subscriber::fmt::init();

    info!("Bootstrapping AetherFabric Node Agent");

    // Temporary static node ID
    //
    // Later this can be dynamically assigned
    // using hostname, UUID, configuration,
    // or orchestration discovery.
    let node_id = String::from("PI-01");

    // Create runtime instance
    let mut runtime = Runtime::new(node_id);

    // Start runtime loop
    if let Err(e) = runtime.start().await {
        error!("Runtime failure: {:?}", e);
    }

    Ok(())
}