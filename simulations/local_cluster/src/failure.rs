// simulations/local_cluster/src/failure.rs

use crate::cluster::LocalCluster;
use crate::node::SimulatedNode;

use rand::Rng;

use tracing::{info, warn};

/// Represents simulated failure types.
///
/// Used for:
/// - orchestration resilience testing
/// - failover experiments
/// - distributed runtime validation
#[derive(Debug, Clone, Copy)]
pub enum FailureType {
    /// Node becomes completely unreachable
    NodeCrash,

    /// Thermal overload event
    ThermalOverload,

    /// Artificial network partition
    NetworkPartition,

    /// Temporary compute stall
    ComputeStall,
}

/// Represents failure injection event.
#[derive(Debug, Clone)]
pub struct FailureEvent {
    /// Target node
    pub node_id: String,

    /// Failure classification
    pub failure_type: FailureType,

    /// Duration in seconds
    pub duration_seconds: u64,
}

impl FailureEvent {
    /// Creates failure event.
    pub fn new(
        node_id: String,
        failure_type: FailureType,
        duration_seconds: u64,
    ) -> Self {
        Self {
            node_id,
            failure_type,
            duration_seconds,
        }
    }

    /// Returns formatted summary.
    pub fn summary(&self) -> String {
        format!(
            "\
Failure Event
-------------------------
Node ID  : {}
Failure  : {:?}
Duration : {}s
",
            self.node_id,
            self.failure_type,
            self.duration_seconds,
        )
    }
}

/// Responsible for:
/// - simulated runtime failures
/// - orchestration resilience testing
/// - failover experimentation
///
/// IMPORTANT:
/// This layer exists ONLY for:
/// - simulation
/// - distributed systems validation
pub struct FailureInjector;

impl FailureInjector {
    /// Ensures node is online before injecting failures.
    fn ensure_online(node: &SimulatedNode) -> bool {
        if !node.online {
            warn!("Aborting failure injection: Node [{}] is offline", node.node_id);
            false
        } else {
            true
        }
    }

    /// Injects failure into cluster.
    pub fn inject(
        cluster: &mut LocalCluster,
        event: FailureEvent,
    ) {
        warn!("========================================");

        warn!("FAILURE INJECTION");

        warn!("========================================");

        warn!("\n{}", event.summary());

        match event.failure_type {
            FailureType::NodeCrash => {
                Self::inject_node_crash(
                    cluster,
                    &event.node_id,
                );
            }

            FailureType::ThermalOverload => {
                Self::inject_thermal_overload(
                    cluster,
                    &event.node_id,
                );
            }

            FailureType::NetworkPartition => {
                Self::inject_network_partition(
                    cluster,
                    &event.node_id,
                );
            }

            FailureType::ComputeStall => {
                Self::inject_compute_stall(
                    cluster,
                    &event.node_id,
                );
            }
        }
    }

    /// Simulates complete node crash.
    fn inject_node_crash(
        cluster: &mut LocalCluster,
        node_id: &str,
    ) {
        warn!(
            "Simulating node crash [{}]",
            node_id
        );

        cluster.fail_node(node_id);

        cluster.reconcile_state();
    }

    /// Simulates thermal overload.
    fn inject_thermal_overload(
        cluster: &mut LocalCluster,
        node_id: &str,
    ) {
        if let Some(node) =
            cluster.node_mut(node_id)
        {
            if !Self::ensure_online(node) {
                return;
            }

            warn!(
                "Injecting thermal overload [{}]",
                node_id
            );

            node.temperature = 92.0;

            // Leadership migration trigger
            if node.is_leader {
                warn!(
                    "Leader overheating -> reelection triggered"
                );

                node.is_leader = false;

                cluster.reconcile_state();
            }
        }
    }

    /// Simulates network partition.
    ///
    /// Current implementation:
    /// marks node offline logically.
    ///
    /// Future:
    /// actual transport isolation.
    fn inject_network_partition(
        cluster: &mut LocalCluster,
        node_id: &str,
    ) {
        warn!(
            "Simulating network partition [{}]",
            node_id
        );

        cluster.fail_node(node_id);

        cluster.reconcile_state();
    }

    /// Simulates temporary compute stall.
    fn inject_compute_stall(
        cluster: &mut LocalCluster,
        node_id: &str,
    ) {
        if let Some(node) =
            cluster.node_mut(node_id)
        {
            if !Self::ensure_online(node) {
                return;
            }

            warn!(
                "Injecting compute stall [{}]",
                node_id
            );

            node.cpu_usage = 99.0;
        }
    }

    /// Randomly selects node
    /// and injects random failure.
    pub fn random_failure(
        cluster: &mut LocalCluster,
    ) {
        let online_nodes =
            cluster.online_nodes();

        if online_nodes.is_empty() {
            warn!(
                "No online nodes available for failure injection"
            );

            return;
        }

        let mut rng =
            rand::thread_rng();

        let target =
            online_nodes
                [rng.gen_range(
                    0..online_nodes.len(),
                )]
            .node_id
            .clone();

        let failure =
            match rng.gen_range(0..4) {
                0 => FailureType::NodeCrash,

                1 => {
                    FailureType::ThermalOverload
                }

                2 => {
                    FailureType::NetworkPartition
                }

                _ => FailureType::ComputeStall,
            };

        let event = FailureEvent::new(
            target,
            failure,
            10,
        );

        Self::inject(cluster, event);
    }

    /// Simulates recovery after failure.
    pub fn recover_node(
        cluster: &mut LocalCluster,
        node_id: &str,
    ) {
        info!(
            "Recovering node [{}]",
            node_id
        );

        cluster.recover_node(node_id);

        cluster.reconcile_state();
    }
}