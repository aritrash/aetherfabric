// runtime/node_agent/src/node.rs

use crate::state::NodeStatus;
use crate::telemetry::Telemetry;

use serde::{Deserialize, Serialize};

/// Represents a single compute node inside the
/// AetherFabric distributed runtime.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Node {
    /// Unique node identifier
    pub id: String,

    /// Current operational state
    pub status: NodeStatus,

    /// Latest telemetry snapshot
    pub telemetry: Telemetry,

    /// Indicates whether this node is currently
    /// acting as orchestration leader
    pub is_leader: bool,
}

impl Node {
    /// Creates a new node instance.
    pub fn new(id: String) -> Self {
        Self {
            id,
            status: NodeStatus::Initializing,
            telemetry: Telemetry::collect(),
            is_leader: false,
        }
    }

    /// Refreshes telemetry data from the local system.
    pub fn update_telemetry(&mut self) {
        self.telemetry = Telemetry::collect();
    }

    /// Updates runtime state based on current telemetry.
    pub fn evaluate_status(&mut self) {
        let cpu_temp = self.telemetry.cpu_temperature_c;
        let cpu_usage = self.telemetry.cpu_usage;

        self.status = if cpu_temp >= 80.0 {
            NodeStatus::ThermalCritical
        } else if cpu_usage >= 85.0 {
            NodeStatus::Overloaded
        } else if cpu_usage >= 40.0 {
            NodeStatus::Busy
        } else {
            NodeStatus::Active
        };
    }

    /// Sets orchestration leadership state.
    pub fn set_leader(&mut self, leader: bool) {
        self.is_leader = leader;
    }

    /// Returns formatted runtime summary for logging/UI.
    pub fn summary(&self) -> String {
        format!(
            "\
Node ID           : {}
Status            : {}
Leader            : {}
CPU Usage         : {:.2}%
Memory Usage      : {} MB
Available Memory  : {} MB
CPU Temperature   : {:.2} °C
Timestamp         : {}
",
            self.id,
            self.status,
            self.is_leader,
            self.telemetry.cpu_usage,
            self.telemetry.memory_usage_mb,
            self.telemetry.available_memory_mb,
            self.telemetry.cpu_temperature_c,
            self.telemetry.timestamp,
        )
    }
}
