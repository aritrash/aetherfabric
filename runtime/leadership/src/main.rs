// runtime/leadership/src/main.rs

use leadership::election::ElectionEngine;
use node_agent::node::Node;

use tracing::info;

fn main() {
    // Initialize logging
    tracing_subscriber::fmt::init();

    info!("========================================");
    info!("AetherFabric Leadership Engine");
    info!("========================================");

    // -------------------------------------------------
    // Simulated Node Fabric
    // -------------------------------------------------

    let mut pi_01 = Node::new("PI-01".to_string());
    let mut pi_02 = Node::new("PI-02".to_string());
    let mut pi_03 = Node::new("PI-03".to_string());

    // -------------------------------------------------
    // Simulated Telemetry Overrides
    // -------------------------------------------------

    pi_01.telemetry.cpu_usage = 28.0;
    pi_01.telemetry.cpu_temperature_c = 49.0;
    pi_01.telemetry.available_memory_mb = 3200;

    pi_02.telemetry.cpu_usage = 82.0;
    pi_02.telemetry.cpu_temperature_c = 74.0;
    pi_02.telemetry.available_memory_mb = 1800;

    pi_03.telemetry.cpu_usage = 17.0;
    pi_03.telemetry.cpu_temperature_c = 43.0;
    pi_03.telemetry.available_memory_mb = 3500;

    // Update node runtime states
    pi_01.evaluate_status();
    pi_02.evaluate_status();
    pi_03.evaluate_status();

    // -------------------------------------------------
    // Fabric Simulation
    // -------------------------------------------------

    let fabric = vec![pi_01, pi_02, pi_03];

    // Run election
    let (leader_state, scores) = ElectionEngine::elect(&fabric);

    // -------------------------------------------------
    // Output Leadership State
    // -------------------------------------------------

    info!("\n{}", leader_state.summary());

    // -------------------------------------------------
    // Output Node Scores
    // -------------------------------------------------

    info!("========================================");
    info!("Leadership Scoreboard");
    info!("========================================");

    for score in scores {
        info!("\n{}", score.summary());
    }

    info!("========================================");
    info!("Election Completed");
    info!("========================================");
}
