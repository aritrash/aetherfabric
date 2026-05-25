// runtime/fabric/src/registry.rs

use std::collections::HashMap;

use node_agent::node::Node;
use node_agent::state::NodeStatus;

use tracing::{info, warn};

/// Central node registry for the AetherFabric runtime.
///
/// Responsible for:
/// - node tracking
/// - orchestration visibility
/// - node lookup
/// - liveness management
#[derive(Debug)]
pub struct NodeRegistry {
    /// Registered orchestration nodes
    nodes: HashMap<String, Node>,
}

impl NodeRegistry {
    /// Creates empty registry.
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
        }
    }

    /// Registers a node into the fabric.
    pub fn register(&mut self, node: Node) {
        info!("Registered node [{}]", node.id);

        self.nodes.insert(node.id.clone(), node);
    }

    /// Removes a node from registry.
    pub fn remove(&mut self, node_id: &str) -> Option<Node> {
        warn!("Removing node [{}]", node_id);

        self.nodes.remove(node_id)
    }

    /// Retrieves immutable node reference.
    pub fn get(&self, node_id: &str) -> Option<&Node> {
        self.nodes.get(node_id)
    }

    /// Retrieves mutable node reference.
    pub fn get_mut(&mut self, node_id: &str) -> Option<&mut Node> {
        self.nodes.get_mut(node_id)
    }

    /// Returns all active nodes.
    pub fn active_nodes(&self) -> Vec<&Node> {
        self.nodes
            .values()
            .filter(|node| node.status != NodeStatus::Offline)
            .collect()
    }

    /// Returns all registered nodes.
    pub fn all_nodes(&self) -> Vec<&Node> {
        self.nodes.values().collect()
    }

    /// Returns registry size.
    pub fn total_nodes(&self) -> usize {
        self.nodes.len()
    }

    /// Marks node offline.
    pub fn mark_offline(&mut self, node_id: &str) {
        if let Some(node) = self.nodes.get_mut(node_id) {
            node.status = NodeStatus::Offline;

            warn!("Node [{}] marked OFFLINE", node_id);
        }
    }

    /// Returns formatted registry summary.
    pub fn summary(&self) -> String {
        let mut output = String::new();

        output.push_str("========================================\n");

        output.push_str("NODE REGISTRY STATE\n");

        output.push_str("========================================\n\n");

        output.push_str(&format!("Registered Nodes: {}\n\n", self.total_nodes()));

        for node in self.nodes.values() {
            output.push_str(&format!("Node [{}] -> {}\n", node.id, node.status));
        }

        output
    }
}
