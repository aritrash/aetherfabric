// simulations/local_cluster/src/topology.rs

use std::collections::HashMap;

use tracing::{info, warn};

/// Represents logical node connection.
///
/// Used for:
/// - peer visibility
/// - simulated network graph
/// - orchestration topology analysis
#[derive(Debug, Clone)]
pub struct NodeLink {
    /// Source node
    pub source: String,

    /// Destination node
    pub destination: String,

    /// Simulated network latency (ms)
    pub latency_ms: u32,

    /// Link operational state
    pub active: bool,
}

impl NodeLink {
    /// Creates topology link.
    pub fn new(
        source: String,
        destination: String,
        latency_ms: u32,
    ) -> Self {
        Self {
            source,
            destination,
            latency_ms,
            active: true,
        }
    }

    /// Returns formatted summary.
    pub fn summary(&self) -> String {
        format!(
            "{} -> {} | LATENCY={}ms | ACTIVE={}",
            self.source,
            self.destination,
            self.latency_ms,
            self.active,
        )
    }
}

/// Represents simulated cluster topology.
///
/// This layer models:
/// - peer connectivity
/// - network graph structure
/// - simulated transport visibility
///
/// IMPORTANT:
/// This is:
/// - logical topology only
/// - not real transport routing
/// - simulation-oriented
#[derive(Debug)]
pub struct ClusterTopology {
    /// Registered topology links
    pub links: Vec<NodeLink>,

    /// Adjacency map
    adjacency:
        HashMap<String, Vec<String>>,
}

impl ClusterTopology {
    /// Creates empty topology.
    pub fn new() -> Self {
        Self {
            links: Vec::new(),
            adjacency:
                HashMap::new(),
        }
    }

    /// Registers bidirectional link.
    pub fn connect(
        &mut self,
        node_a: String,
        node_b: String,
        latency_ms: u32,
    ) {
        info!(
            "Connecting [{}] <-> [{}]",
            node_a,
            node_b
        );

        // -------------------------------------------------
        // Forward Link
        // -------------------------------------------------

        self.links.push(
            NodeLink::new(
                node_a.clone(),
                node_b.clone(),
                latency_ms,
            ),
        );

        // -------------------------------------------------
        // Reverse Link
        // -------------------------------------------------

        self.links.push(
            NodeLink::new(
                node_b.clone(),
                node_a.clone(),
                latency_ms,
            ),
        );

        // -------------------------------------------------
        // Adjacency Registration
        // -------------------------------------------------

        self.adjacency
            .entry(node_a.clone())
            .or_default()
            .push(node_b.clone());

        self.adjacency
            .entry(node_b)
            .or_default()
            .push(node_a);
    }

    /// Disconnects logical topology link.
    pub fn disconnect(
        &mut self,
        node_a: &str,
        node_b: &str,
    ) {
        warn!(
            "Disconnecting [{}] <-> [{}]",
            node_a,
            node_b
        );

        self.links.retain(|link| {
            !(
                (link.source == node_a
                    && link.destination
                        == node_b)
                    || (link.source
                        == node_b
                        && link.destination
                            == node_a)
            )
        });

        if let Some(peers) =
            self.adjacency.get_mut(node_a)
        {
            peers.retain(|peer| {
                peer != node_b
            });
        }

        if let Some(peers) =
            self.adjacency.get_mut(node_b)
        {
            peers.retain(|peer| {
                peer != node_a
            });
        }
    }

    /// Returns peers for node.
    pub fn peers(
        &self,
        node_id: &str,
    ) -> Vec<String> {
        self.adjacency
            .get(node_id)
            .cloned()
            .unwrap_or_default()
    }

    /// Returns whether nodes are connected.
    pub fn connected(
        &self,
        node_a: &str,
        node_b: &str,
    ) -> bool {
        self.links.iter().any(|link| {
            link.source == node_a
                && link.destination
                    == node_b
                && link.active
        })
    }

    /// Returns total topology links.
    pub fn total_links(&self) -> usize {
        self.links.len()
    }

    /// Simulates network partition.
    pub fn partition_node(
        &mut self,
        node_id: &str,
    ) {
        warn!(
            "Partitioning node [{}]",
            node_id
        );

        for link in self.links.iter_mut() {
            if link.source == node_id
                || link.destination
                    == node_id
            {
                link.active = false;
            }
        }
    }

    /// Restores network partition.
    pub fn restore_node(
        &mut self,
        node_id: &str,
    ) {
        info!(
            "Restoring connectivity [{}]",
            node_id
        );

        for link in self.links.iter_mut() {
            if link.source == node_id
                || link.destination
                    == node_id
            {
                link.active = true;
            }
        }
    }

    /// Returns formatted topology summary.
    pub fn summary(&self) -> String {
        let mut output = String::new();

        output.push_str(
            "========================================\n",
        );

        output.push_str(
            "CLUSTER TOPOLOGY\n",
        );

        output.push_str(
            "========================================\n\n",
        );

        output.push_str(
            &format!(
                "Total Links : {}\n\n",
                self.total_links()
            ),
        );

        output.push_str(
            "Topology Links\n",
        );

        output.push_str(
            "----------------------------------------\n",
        );

        for link in &self.links {
            output.push_str(
                &format!(
                    "{}\n",
                    link.summary()
                ),
            );
        }

        output.push('\n');

        output
    }

    /// Creates default fully-connected
    /// 3-node topology.
    pub fn default_topology() -> Self {
        let mut topology =
            Self::new();

        topology.connect(
            "PI-01".to_string(),
            "PI-02".to_string(),
            2,
        );

        topology.connect(
            "PI-02".to_string(),
            "PI-03".to_string(),
            2,
        );

        topology.connect(
            "PI-01".to_string(),
            "PI-03".to_string(),
            1,
        );

        topology
    }
}