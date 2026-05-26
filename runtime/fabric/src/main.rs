// runtime/fabric/src/main.rs

mod fabric;
mod heartbeat;
mod registry;
mod sync;

use fabric::Fabric;
use heartbeat::Heartbeat;
use sync::SyncEngine;

use node_agent::node::Node;

use tracing::{info, warn};

fn main() {
    // Initialize logging
    tracing_subscriber::fmt::init();

    info!("========================================");
    info!("AetherFabric Fabric Runtime");
    info!("========================================");

    // -------------------------------------------------
    // Create Simulated Fabric
    // -------------------------------------------------

    let mut fabric = Fabric::new();

    // -------------------------------------------------
    // Create Simulated Nodes
    // -------------------------------------------------

    let mut pi_01 = Node::new("PI-01".to_string());
    let mut pi_02 = Node::new("PI-02".to_string());
    let mut pi_03 = Node::new("PI-03".to_string());

    // -------------------------------------------------
    // Simulated Telemetry States
    // -------------------------------------------------

    pi_01.telemetry.cpu_usage = 22.0;
    pi_01.telemetry.cpu_temperature_c = 48.0;
    pi_01.telemetry.available_memory_mb = 3200;

    pi_02.telemetry.cpu_usage = 79.0;
    pi_02.telemetry.cpu_temperature_c = 74.0;
    pi_02.telemetry.available_memory_mb = 1800;

    pi_03.telemetry.cpu_usage = 15.0;
    pi_03.telemetry.cpu_temperature_c = 43.0;
    pi_03.telemetry.available_memory_mb = 3500;

    // Evaluate node states
    pi_01.evaluate_status();
    pi_02.evaluate_status();
    pi_03.evaluate_status();

    // -------------------------------------------------
    // Register Nodes Into Fabric
    // -------------------------------------------------

    fabric.register_node(pi_01.clone());
    fabric.register_node(pi_02.clone());
    fabric.register_node(pi_03.clone());

    info!("\n{}", fabric.summary());

    // -------------------------------------------------
    // Generate Heartbeats
    // -------------------------------------------------

    let hb_01 = Heartbeat::new(
        pi_01.id.clone(),
        pi_01.telemetry.clone(),
        pi_01.status.clone(),
        false,
    );

    let hb_02 = Heartbeat::new(
        pi_02.id.clone(),
        pi_02.telemetry.clone(),
        pi_02.status.clone(),
        false,
    );

    let hb_03 = Heartbeat::new(
        pi_03.id.clone(),
        pi_03.telemetry.clone(),
        pi_03.status.clone(),
        true,
    );

    // -------------------------------------------------
    // Synchronize Heartbeats
    // -------------------------------------------------

    SyncEngine::synchronize_heartbeat(&mut fabric, hb_01);

    SyncEngine::synchronize_heartbeat(&mut fabric, hb_02);

    SyncEngine::synchronize_heartbeat(&mut fabric, hb_03);

    // -------------------------------------------------
    // Simulate Leader Update
    // -------------------------------------------------

    fabric.update_leader("PI-03".to_string(), current_timestamp());

    info!("========================================");
    info!("Updated Fabric State");
    info!("========================================");

    info!("\n{}", fabric.summary());

    // -------------------------------------------------
    // Simulate Synchronization Cycle
    // -------------------------------------------------

    SyncEngine::synchronize_fabric(&mut fabric);

    // -------------------------------------------------
    // Simulate Node Failure
    // -------------------------------------------------

    warn!("========================================");
    warn!("Simulating Node Failure");
    warn!("========================================");

    fabric.registry.mark_offline("PI-02");

    info!("\n{}", fabric.registry.summary());

    // -------------------------------------------------
    // Re-run Synchronization
    // -------------------------------------------------

    SyncEngine::synchronize_fabric(&mut fabric);

    info!("========================================");
    info!("Fabric Runtime Simulation Complete");
    info!("========================================");
}

/// Returns current UNIX timestamp.
fn current_timestamp() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};

    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}
