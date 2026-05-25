// simulations/local_cluster/src/scenario.rs

use crate::cluster::LocalCluster;
use crate::failure::{
    FailureEvent,
    FailureInjector,
    FailureType,
};
use crate::launcher::ClusterLauncher;

use tracing::{info, warn};

/// Represents supported orchestration scenarios.
///
/// These scenarios simulate:
/// - distributed behavior
/// - runtime instability
/// - orchestration recovery
/// - leadership migration
#[derive(Debug, Clone, Copy)]
pub enum SimulationScenario {
    /// Stable cluster operation
    StableCluster,

    /// Leader node failure
    LeaderFailure,

    /// Thermal overload event
    ThermalMigration,

    /// Randomized runtime failure
    RandomFailure,

    /// Full cluster shutdown
    ClusterShutdown,
}

/// Scenario execution engine.
///
/// Responsible for:
/// - orchestration experimentation
/// - distributed systems validation
/// - resilience testing
/// - runtime behavior analysis
pub struct ScenarioRunner;

impl ScenarioRunner {
    /// Executes simulation scenario.
    pub fn execute(
        cluster: &mut LocalCluster,
        scenario: SimulationScenario,
    ) {
        info!("========================================");

        info!(
            "Executing Scenario [{:?}]",
            scenario
        );

        info!("========================================");

        match scenario {
            SimulationScenario::StableCluster => {
                Self::stable_cluster(
                    cluster,
                );
            }

            SimulationScenario::LeaderFailure => {
                Self::leader_failure(
                    cluster,
                );
            }

            SimulationScenario::ThermalMigration => {
                Self::thermal_migration(
                    cluster,
                );
            }

            SimulationScenario::RandomFailure => {
                Self::random_failure(
                    cluster,
                );
            }

            SimulationScenario::ClusterShutdown => {
                Self::cluster_shutdown(
                    cluster,
                );
            }
        }
    }

    /// Simulates healthy cluster runtime.
    fn stable_cluster(
        cluster: &mut LocalCluster,
    ) {
        info!(
            "Running stable orchestration cycle"
        );

        ClusterLauncher::run_simulation_loop(
            cluster,
            5,
            std::time::Duration::from_secs(1),
        );

        info!("\n{}", cluster.summary());
    }

    /// Simulates leader crash and reelection.
    fn leader_failure(
        cluster: &mut LocalCluster,
    ) {
        let leader =
            cluster
                .current_leader()
                .map(|node| {
                    node.node_id.clone()
                });

        if let Some(leader_id) = leader {
            warn!(
                "Injecting leader failure [{}]",
                leader_id
            );

            let failure =
                FailureEvent::new(
                    leader_id,
                    FailureType::NodeCrash,
                    15,
                );

            FailureInjector::inject(
                cluster,
                failure,
            );

            info!(
                "\n{}",
                cluster.summary()
            );
        } else {
            warn!(
                "No leader available for failure simulation"
            );
        }
    }

    /// Simulates thermal overload
    /// triggering orchestration migration.
    fn thermal_migration(
        cluster: &mut LocalCluster,
    ) {
        let leader =
            cluster
                .current_leader()
                .map(|node| {
                    node.node_id.clone()
                });

        if let Some(leader_id) = leader {
            warn!(
                "Injecting thermal overload [{}]",
                leader_id
            );

            let event =
                FailureEvent::new(
                    leader_id,
                    FailureType::ThermalOverload,
                    20,
                );

            FailureInjector::inject(
                cluster,
                event,
            );

            info!(
                "\n{}",
                cluster.summary()
            );
        }
    }

    /// Simulates randomized runtime instability.
    fn random_failure(
        cluster: &mut LocalCluster,
    ) {
        warn!(
            "Executing randomized failure injection"
        );

        FailureInjector::random_failure(
            cluster,
        );

        info!(
            "\n{}",
            cluster.summary()
        );
    }

    /// Simulates graceful cluster shutdown.
    fn cluster_shutdown(
        cluster: &mut LocalCluster,
    ) {
        warn!(
            "Executing cluster shutdown sequence"
        );

        ClusterLauncher::shutdown(
            cluster,
        );
    }
}