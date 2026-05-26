// runtime/node_agent/src/state.rs

use serde::{Deserialize, Serialize};
use std::fmt;

/// Represents the current operational state of a node
/// inside the AetherFabric runtime.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum NodeStatus {
    Initializing,
    Active,
    Busy,
    Overloaded,
    ThermalCritical,
    Offline,
}

impl fmt::Display for NodeStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let status = match self {
            NodeStatus::Initializing => "INITIALIZING",
            NodeStatus::Active => "ACTIVE",
            NodeStatus::Busy => "BUSY",
            NodeStatus::Overloaded => "OVERLOADED",
            NodeStatus::ThermalCritical => "THERMAL_CRITICAL",
            NodeStatus::Offline => "OFFLINE",
        };

        write!(f, "{status}")
    }
}
