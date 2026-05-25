// runtime/fabric/src/fabric.rs

use crate::heartbeat::Heartbeat;
use crate::registry::NodeRegistry;

use leadership::leader::LeaderState;

use node_agent::node::Node;

use tracing::{info, warn};

/// Represents the global orchestration state
/// of the AetherFabric runtime.
///
/// The Fabric is responsible for:
/// - maintaining node awareness
/// - tracking orchestration leadership
/// - synchronizing telemetry state
/// - monitoring node liveness
///
/// This is the distributed orchestration layer
/// above individual node runtimes.
#[derive(Debug)]
pub struct Fabric {
    /// Known node registry
    pub registry: NodeRegistry,

    /// Current orchestration leadership state
    pub leader_state: LeaderState,
}

impl Fabric {
    /// Creates an empty fabric instance.
    pub fn new() -> Self {
        Self {
            registry: NodeRegistry::new(),
            leader_state: LeaderState::new(),
        }
    }

    /// Registers a node into the fabric.
    pub fn register_node(&mut self, node: Node) {
        info!("Registering node [{}] into fabric", node.id);

        self.registry.register(node);
    }

    /// Processes incoming heartbeat update.
    ///
    /// Heartbeats refresh:
    /// - telemetry
    /// - node liveness
    /// - orchestration awareness
    pub fn process_heartbeat(&mut self, heartbeat: Heartbeat) {
        info!("Processing heartbeat from [{}]", heartbeat.node_id);

        match self.registry.get_mut(&heartbeat.node_id) {
            Some(node) => {
                node.telemetry = heartbeat.telemetry;
                node.evaluate_status();

                info!("Updated node [{}] telemetry", node.id);
            }

            None => {
                warn!(
                    "Received heartbeat from unknown node [{}]",
                    heartbeat.node_id
                );
            }
        }
    }

    /// Updates orchestration leadership state.
    pub fn update_leader(&mut self, leader_id: String, timestamp: u64) {
        self.leader_state.update_leader(leader_id, timestamp);
    }

    /// Returns active node count.
    pub fn active_nodes(&self) -> usize {
        self.registry.active_nodes().len()
    }

    /// Returns total registered nodes.
    pub fn total_nodes(&self) -> usize {
        self.registry.total_nodes()
    }

    /// Returns formatted fabric summary.
    pub fn summary(&self) -> String {
        format!(
            "\
========================================
AETHERFABRIC STATE
========================================

Registered Nodes : {}
Active Nodes     : {}

{}
",
            self.total_nodes(),
            self.active_nodes(),
            self.leader_state.summary(),
        )
    }
}
