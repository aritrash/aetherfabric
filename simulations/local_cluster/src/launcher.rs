// simulations/local_cluster/src/launcher.rs

use crate::cluster::LocalCluster;
use crate::node::SimulatedNode;

use std::time::Duration;
use std::thread;

use tracing::{info, warn};

/// Responsible for:
/// - cluster bootstrapping
/// - node spawning
/// - topology initialization
/// - orchestration simulation setup
///
/// IMPORTANT:
/// Current implementation is:
/// - process-simulated
/// - localhost-oriented
/// - experimentation-focused
///
/// Future versions may:
/// - spawn real subprocesses
/// - launch runtime binaries
/// - orchestrate QEMU instances
pub struct ClusterLauncher;

impl ClusterLauncher {
    /// Creates default 3-node cluster.
    ///
    /// Simulated topology:
    ///
    /// PI-01
    /// PI-02
    /// PI-03
    pub fn default_cluster() -> LocalCluster {
        info!("========================================");

        info!(
            "Bootstrapping Default Local Cluster"
        );

        info!("========================================");

        let mut cluster =
            LocalCluster::new(
                "AetherFabric-LocalCluster"
                    .to_string(),
            );

        // -------------------------------------------------
        // PI-01
        // -------------------------------------------------

        let mut pi_01 =
            SimulatedNode::new(
                "PI-01".to_string(),
                9001,
                cluster.clock.current_time_secs,
            );

        pi_01.cpu_usage = 22.0;
        pi_01.temperature = 48.0;

        cluster.register_node(pi_01);

        // -------------------------------------------------
        // PI-02
        // -------------------------------------------------

        let mut pi_02 =
            SimulatedNode::new(
                "PI-02".to_string(),
                9002,
                cluster.clock.current_time_secs,
            );

        pi_02.cpu_usage = 64.0;
        pi_02.temperature = 62.0;

        cluster.register_node(pi_02);

        // -------------------------------------------------
        // PI-03
        // -------------------------------------------------

        let mut pi_03 =
            SimulatedNode::new(
                "PI-03".to_string(),
                9003,
                cluster.clock.current_time_secs,
            );

        pi_03.cpu_usage = 15.0;
        pi_03.temperature = 43.0;

        cluster.register_node(pi_03);

        // -------------------------------------------------
        // Initial Leader Election
        // -------------------------------------------------

        cluster.reconcile_state();

        cluster
    }

    /// Starts cluster simulation runtime.
    pub fn launch(
        cluster: &mut LocalCluster,
    ) {
        info!("========================================");

        info!(
            "Launching LocalCluster Runtime"
        );

        info!("========================================");

        cluster.start_cluster();

        info!("\n{}", cluster.summary());
    }

    /// Stops cluster simulation runtime.
    pub fn shutdown(
        cluster: &mut LocalCluster,
    ) {
        warn!("========================================");

        warn!(
            "Shutting Down LocalCluster Runtime"
        );

        warn!("========================================");

        cluster.stop_cluster();

        info!("\n{}", cluster.summary());
    }

    /// Simulates orchestration cycle.
    ///
    /// Future responsibilities:
    /// - heartbeat propagation
    /// - runtime synchronization
    /// - scheduler simulation
    /// - telemetry evolution
    pub fn simulation_tick(
        cluster: &mut LocalCluster,
    ) {
        info!("========================================");
        info!("Executing Simulation Tick");
        info!("========================================");

        for node in cluster.nodes.values_mut() {
            node.tick(cluster.clock.current_time_secs);
        }

        // Reevaluate leadership after state changes
        cluster.reconcile_state();
    }

    /// Centralized execution loop with timing semantics.
    pub fn run_simulation_loop(
        cluster: &mut LocalCluster,
        ticks: u64,
        interval: Duration,
    ) {
        info!("========================================");
        info!("Starting Simulation Loop ({} ticks, {:?} interval)", ticks, interval);
        info!("========================================");

        for _ in 0..ticks {
            cluster.clock.advance(interval.as_secs());
            Self::simulation_tick(cluster);
            thread::sleep(interval);
        }
    }
}