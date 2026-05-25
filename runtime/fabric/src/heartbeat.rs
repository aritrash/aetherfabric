// runtime/fabric/src/heartbeat.rs

use serde::{Deserialize, Serialize};

use std::time::{SystemTime, UNIX_EPOCH};

use node_agent::state::NodeStatus;
use node_agent::telemetry::Telemetry;

/// Represents a distributed runtime heartbeat.
///
/// Heartbeats synchronize:
/// - node telemetry
/// - node runtime state
/// - orchestration awareness
/// - node liveness
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Heartbeat {
    /// Source node ID
    pub node_id: String,

    /// Current node telemetry snapshot
    pub telemetry: Telemetry,

    /// Current runtime status
    pub status: NodeStatus,

    /// Whether node currently acts as leader
    pub is_leader: bool,

    /// Heartbeat generation timestamp
    pub timestamp: u64,
}

impl Heartbeat {
    /// Creates a heartbeat packet from local node state.
    pub fn new(node_id: String, telemetry: Telemetry, status: NodeStatus, is_leader: bool) -> Self {
        Self {
            node_id,
            telemetry,
            status,
            is_leader,
            timestamp: current_timestamp(),
        }
    }

    /// Returns formatted heartbeat summary.
    pub fn summary(&self) -> String {
        format!(
            "\
Heartbeat Packet
-------------------------
Node ID        : {}
Status         : {}
Leader         : {}
CPU Usage      : {:.2}%
Temperature    : {:.2} °C
Timestamp      : {}
",
            self.node_id,
            self.status,
            self.is_leader,
            self.telemetry.cpu_usage,
            self.telemetry.cpu_temperature_c,
            self.timestamp,
        )
    }
}

/// Returns current UNIX timestamp.
fn current_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}
