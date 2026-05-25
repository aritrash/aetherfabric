// simulations/local_cluster/src/events.rs

use crate::health::ClusterHealth;

/// Represents significant state changes in the simulated cluster
#[derive(Debug, Clone)]
pub enum OrchestrationEvent {
    LeaderChanged {
        timestamp: u64,
        old_leader: Option<String>,
        new_leader: Option<String>,
    },
    NodeOffline {
        timestamp: u64,
        node_id: String,
    },
    ThermalMigration {
        timestamp: u64,
        node_id: String,
    },
    HeartbeatTimeout {
        timestamp: u64,
        node_id: String,
    },
    ReconciliationPerformed {
        timestamp: u64,
        actions_taken: usize,
    },
    DegradedLeadership {
        timestamp: u64,
        node_id: String,
    },
    ClusterHealthChanged {
        timestamp: u64,
        old: ClusterHealth,
        new: ClusterHealth,
    },
}

impl OrchestrationEvent {
    /// Helper to get the event timestamp
    pub fn timestamp(&self) -> u64 {
        match self {
            Self::LeaderChanged { timestamp, .. } => *timestamp,
            Self::NodeOffline { timestamp, .. } => *timestamp,
            Self::ThermalMigration { timestamp, .. } => *timestamp,
            Self::HeartbeatTimeout { timestamp, .. } => *timestamp,
            Self::ReconciliationPerformed { timestamp, .. } => *timestamp,
            Self::DegradedLeadership { timestamp, .. } => *timestamp,
            Self::ClusterHealthChanged { timestamp, .. } => *timestamp,
        }
    }
}
