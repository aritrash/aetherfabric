// simulations/local_cluster/src/main.rs

mod cluster;
pub mod clock;
pub mod health;
mod events;
mod failure;
mod launcher;
mod node;
mod scenario;
mod topology;

use launcher::ClusterLauncher;

use scenario::{
    ScenarioRunner,
    SimulationScenario,
};

use topology::ClusterTopology;

use tracing::info;

use std::{
    thread,
    time::Duration,
};

fn main() {
    // -------------------------------------------------
    // Logging Initialization
    // -------------------------------------------------

    tracing_subscriber::fmt::init();

    info!("========================================");

    info!(
        "AetherFabric LocalCluster Simulation"
    );

    info!("========================================");

    // -------------------------------------------------
    // Cluster Bootstrap
    // -------------------------------------------------

    let mut cluster =
        ClusterLauncher::default_cluster();

    // -------------------------------------------------
    // Topology Initialization
    // -------------------------------------------------

    let topology =
        ClusterTopology::default_topology();

    info!("\n{}", topology.summary());

    // -------------------------------------------------
    // Launch Cluster
    // -------------------------------------------------

    ClusterLauncher::launch(
        &mut cluster,
    );

    // -------------------------------------------------
    // Stable Cluster Scenario
    // -------------------------------------------------

    ScenarioRunner::execute(
        &mut cluster,
        SimulationScenario::StableCluster,
    );

    thread::sleep(
        Duration::from_secs(2),
    );

    // -------------------------------------------------
    // Leader Failure Scenario
    // -------------------------------------------------

    ScenarioRunner::execute(
        &mut cluster,
        SimulationScenario::LeaderFailure,
    );

    thread::sleep(
        Duration::from_secs(2),
    );

    // -------------------------------------------------
    // Thermal Migration Scenario
    // -------------------------------------------------

    ScenarioRunner::execute(
        &mut cluster,
        SimulationScenario::ThermalMigration,
    );

    thread::sleep(
        Duration::from_secs(2),
    );

    // -------------------------------------------------
    // Random Failure Scenario
    // -------------------------------------------------

    ScenarioRunner::execute(
        &mut cluster,
        SimulationScenario::RandomFailure,
    );

    thread::sleep(
        Duration::from_secs(2),
    );

    // -------------------------------------------------
    // Final Cluster Summary
    // -------------------------------------------------

    info!("========================================");

    info!("FINAL CLUSTER STATE");

    info!("========================================");

    info!("\n{}", cluster.summary());

    // -------------------------------------------------
    // Graceful Shutdown
    // -------------------------------------------------

    ScenarioRunner::execute(
        &mut cluster,
        SimulationScenario::ClusterShutdown,
    );

    info!("========================================");

    info!(
        "Simulation Runtime Complete"
    );

    info!("========================================");
}