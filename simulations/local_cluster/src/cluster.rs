// simulations/local_cluster/src/cluster.rs

use crate::node::SimulatedNode;
use crate::events::OrchestrationEvent;
use crate::clock::SimulationClock;
use crate::health::ClusterHealth;

use std::collections::{HashMap, VecDeque};

use tracing::{info, warn};

/// Represents simulated cluster topology.
///
/// This layer is responsible for:
/// - simulated node management
/// - cluster membership
/// - orchestration visibility
/// - local distributed experiments
///
/// IMPORTANT:
/// This is NOT production runtime logic.
///
/// This exists purely for:
/// - experimentation
/// - validation
/// - orchestration simulation
/// - distributed systems testing
#[derive(Debug)]
pub struct LocalCluster {
    /// Cluster identifier
    pub cluster_name: String,

    /// Simulated nodes
    pub nodes:
        HashMap<String, SimulatedNode>,

    /// Event history
    pub events: VecDeque<OrchestrationEvent>,

    /// Last election timestamp
    pub last_election_timestamp: u64,

    /// Logical simulation clock
    pub clock: SimulationClock,

    /// Overall cluster health state
    pub health: ClusterHealth,
}

impl LocalCluster {
    /// Creates local simulation cluster.
    pub fn new(
        cluster_name: String,
    ) -> Self {
        Self {
            cluster_name,
            nodes: HashMap::new(),
            events: VecDeque::new(),
            last_election_timestamp: 0,
            clock: SimulationClock::new(),
            health: ClusterHealth::Healthy,
        }
    }

    /// Pushes an orchestration event
    pub fn push_event(&mut self, event: OrchestrationEvent) {
        self.events.push_back(event);
        while self.events.len() > 100 {
            self.events.pop_front();
        }
    }

    /// Registers simulated node.
    pub fn register_node(
        &mut self,
        node: SimulatedNode,
    ) {
        info!(
            "Registering node [{}]",
            node.node_id
        );

        self.nodes
            .insert(
                node.node_id.clone(),
                node,
            );
    }

    /// Removes node from cluster.
    pub fn remove_node(
        &mut self,
        node_id: &str,
    ) {
        if self.nodes.remove(node_id).is_some()
        {
            warn!(
                "Removed node [{}]",
                node_id
            );
        }
    }

    /// Returns immutable node reference.
    pub fn node(
        &self,
        node_id: &str,
    ) -> Option<&SimulatedNode> {
        self.nodes.get(node_id)
    }

    /// Returns mutable node reference.
    pub fn node_mut(
        &mut self,
        node_id: &str,
    ) -> Option<&mut SimulatedNode> {
        self.nodes.get_mut(node_id)
    }

    /// Returns all nodes.
    pub fn all_nodes(
        &self,
    ) -> Vec<&SimulatedNode> {
        self.nodes.values().collect()
    }

    /// Returns online nodes.
    pub fn online_nodes(
        &self,
    ) -> Vec<&SimulatedNode> {
        self.nodes
            .values()
            .filter(|node| node.online)
            .collect()
    }

    /// Returns total cluster nodes.
    pub fn total_nodes(&self) -> usize {
        self.nodes.len()
    }

    /// Returns online node count.
    pub fn online_count(&self) -> usize {
        self.nodes
            .values()
            .filter(|node| node.online)
            .count()
    }

    /// Starts all cluster nodes.
    ///
    /// NOTE:
    /// Current implementation is simulated.
    /// Real process spawning comes later.
    pub fn start_cluster(
        &mut self,
    ) {
        info!("========================================");

        info!(
            "Starting LocalCluster [{}]",
            self.cluster_name
        );

        info!("========================================");

        for node in self.nodes.values_mut() {
            node.start(self.clock.current_time_secs);
        }
    }

    /// Stops all cluster nodes.
    pub fn stop_cluster(
        &mut self,
    ) {
        warn!("========================================");

        warn!(
            "Stopping LocalCluster [{}]",
            self.cluster_name
        );

        warn!("========================================");

        for node in self.nodes.values_mut() {
            node.stop();
        }
    }

    /// Simulates node failure.
    pub fn fail_node(
        &mut self,
        node_id: &str,
    ) {
        if let Some(node) =
            self.nodes.get_mut(node_id)
        {
            warn!(
                "Simulating failure [{}]",
                node_id
            );

            node.online = false;
            self.push_event(OrchestrationEvent::NodeOffline {
                timestamp: self.clock.current_time_secs,
                node_id: node_id.to_string(),
            });
        }
        self.reconcile_state();
    }

    /// Simulates node recovery.
    pub fn recover_node(
        &mut self,
        node_id: &str,
    ) {
        if let Some(node) =
            self.nodes.get_mut(node_id)
        {
            info!(
                "Recovering node [{}]",
                node_id
            );

            node.online = true;
        }
        self.reconcile_state();
    }

    /// Returns current cluster leader.
    pub fn current_leader(
        &self,
    ) -> Option<&SimulatedNode> {
        self.nodes
            .values()
            .find(|node| node.is_leader)
    }

    /// Authoritative cluster reconciliation logic.
    /// 1. Timeout detection
    /// 2. Strip invalid leaders
    /// 3. Validate single leader invariant
    /// 4. Heal missing leadership
    /// 5. Emit reconciliation events
    pub fn reconcile_state(&mut self) {
        let now = self.clock.current_time_secs;
        let mut actions_taken = 0;

        // 1. Timeout detection
        const HEARTBEAT_TIMEOUT_SECS: u64 = 5;
        let mut timed_out_nodes = Vec::new();
        for node in self.nodes.values_mut() {
            if node.online {
                let time_since_last_seen = now.saturating_sub(node.last_seen_timestamp);
                if time_since_last_seen >= HEARTBEAT_TIMEOUT_SECS {
                    node.online = false;
                    timed_out_nodes.push(node.node_id.clone());
                    actions_taken += 1;
                }
            }
        }
        for node_id in timed_out_nodes {
            warn!("Heartbeat timeout detected for node [{}]", node_id);
            self.push_event(OrchestrationEvent::HeartbeatTimeout {
                timestamp: now,
                node_id,
            });
        }

        // 2. Strip invalid leaders & 3. Validate single leader invariant
        let mut leader_ids = Vec::new();
        let mut invalid_leaders = Vec::new();
        
        for node in self.nodes.values() {
            if node.is_leader {
                if !node.online || node.temperature >= 85.0 {
                    invalid_leaders.push(node.node_id.clone());
                } else {
                    leader_ids.push(node.node_id.clone());
                }
            }
        }

        // If multiple valid leaders exist, strip all of them to force reelection
        if leader_ids.len() > 1 {
            for id in leader_ids.iter() {
                invalid_leaders.push(id.clone());
            }
            leader_ids.clear();
        }

        for node_id in &invalid_leaders {
            if let Some(node) = self.nodes.get_mut(node_id) {
                node.is_leader = false;
                actions_taken += 1;
                
                if node.temperature >= 85.0 && node.online {
                    self.push_event(OrchestrationEvent::ThermalMigration {
                        timestamp: now,
                        node_id: node_id.clone(),
                    });
                }
            }
        }

        // 4. Heal missing leadership
        let needs_emergency_election = leader_ids.is_empty();
        const MIN_LEADER_STABILITY_SECS: u64 = 10;
        let time_since_election = now.saturating_sub(self.last_election_timestamp);
        let cooldown_passed = time_since_election >= MIN_LEADER_STABILITY_SECS;

        let healthy_candidate = self
            .nodes
            .values()
            .filter(|node| node.online && node.temperature < 85.0)
            .min_by(|a, b| {
                let cpu_penalty_a = if a.cpu_usage >= 95.0 { 1000.0 } else { 0.0 };
                let cpu_penalty_b = if b.cpu_usage >= 95.0 { 1000.0 } else { 0.0 };
                
                let score_a = a.cpu_usage + a.temperature + cpu_penalty_a;
                let score_b = b.cpu_usage + b.temperature + cpu_penalty_b;

                score_a.partial_cmp(&score_b).unwrap()
            })
            .map(|node| node.node_id.clone());

        let current_leader_id = leader_ids.first().cloned();

        if healthy_candidate.is_some() {
            if needs_emergency_election || cooldown_passed {
                let candidate = healthy_candidate;
                if candidate != current_leader_id {
                    for node in self.nodes.values_mut() {
                        node.is_leader = false;
                    }

                    if let Some(ref new_leader_id) = candidate {
                        if let Some(node) = self.nodes.get_mut(new_leader_id) {
                            node.is_leader = true;
                            info!("Leader elected [{}]", new_leader_id);
                        }
                    }
                    
                    self.last_election_timestamp = now;
                    actions_taken += 1;
                    
                    self.push_event(OrchestrationEvent::LeaderChanged {
                        timestamp: now,
                        old_leader: current_leader_id.clone(),
                        new_leader: candidate,
                    });
                }
            }
        } else if self.online_count() > 0 {
            // Degraded emergency fallback
            let degraded_candidate = self
                .nodes
                .values()
                .filter(|node| node.online)
                .min_by(|a, b| a.temperature.partial_cmp(&b.temperature).unwrap())
                .map(|node| node.node_id.clone());

            if degraded_candidate.is_some() && degraded_candidate != current_leader_id {
                for node in self.nodes.values_mut() {
                    node.is_leader = false;
                }

                if let Some(ref new_leader_id) = degraded_candidate {
                    if let Some(node) = self.nodes.get_mut(new_leader_id) {
                        node.is_leader = true;
                        info!("Degraded leader elected [{}]", new_leader_id);
                    }
                }
                
                self.last_election_timestamp = now;
                actions_taken += 1;
                
                self.push_event(OrchestrationEvent::LeaderChanged {
                    timestamp: now,
                    old_leader: current_leader_id.clone(),
                    new_leader: degraded_candidate.clone(),
                });
                
                if let Some(new_leader_id) = degraded_candidate {
                    self.push_event(OrchestrationEvent::DegradedLeadership {
                        timestamp: now,
                        node_id: new_leader_id,
                    });
                }
            }
        }

        // 5. Evaluate Cluster Health
        let online_count = self.online_count();
        let current_leader = self.current_leader();
        
        let new_health = if online_count == 0 {
            ClusterHealth::Dead
        } else if let Some(leader) = current_leader {
            if leader.temperature < 85.0 {
                ClusterHealth::Healthy
            } else {
                ClusterHealth::Degraded
            }
        } else {
            ClusterHealth::Critical
        };
        
        if self.health != new_health {
            self.push_event(OrchestrationEvent::ClusterHealthChanged {
                timestamp: now,
                old: self.health,
                new: new_health,
            });
            self.health = new_health;
            actions_taken += 1;
        }

        // 6. Emit reconciliation event
        if actions_taken > 0 {
            self.push_event(OrchestrationEvent::ReconciliationPerformed {
                timestamp: now,
                actions_taken,
            });
        }
    }

    /// Returns formatted cluster summary.
    pub fn summary(&self) -> String {
        let mut output = String::new();

        output.push_str(
            "========================================\n",
        );

        output.push_str(
            "LOCAL CLUSTER SUMMARY\n",
        );

        output.push_str(
            "========================================\n\n",
        );

        output.push_str(
            &format!(
                "Cluster Name   : {}\n",
                self.cluster_name
            ),
        );

        output.push_str(
            &format!(
                "Cluster Health : {:?}\n\n",
                self.health
            ),
        );

        output.push_str(
            &format!(
                "Total Nodes    : {}\n",
                self.total_nodes()
            ),
        );

        output.push_str(
            &format!(
                "Online Nodes : {}\n\n",
                self.online_count()
            ),
        );

        if let Some(leader) =
            self.current_leader()
        {
            output.push_str(
                &format!(
                    "Leader Node  : {}\n\n",
                    leader.node_id
                ),
            );
        }

        output.push_str(
            "Node State\n",
        );

        output.push_str(
            "----------------------------------------\n",
        );

        for node in self.nodes.values() {
            output.push_str(
                &format!(
                    "{} | ONLINE={} | LEADER={} | CPU={:.2}% | TEMP={:.2}C\n",
                    node.node_id,
                    node.online,
                    node.is_leader,
                    node.cpu_usage,
                    node.temperature,
                ),
            );
        }

        output.push('\n');

        output.push_str("Recent Events\n");
        output.push_str("----------------------------------------\n");
        let start_idx = self.events.len().saturating_sub(5);
        for event in self.events.iter().skip(start_idx) {
            output.push_str(&format!("{:?}\n", event));
        }

        output.push('\n');

        output
    }
}